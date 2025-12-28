macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_chain_for_tuple {
    () => {
        deps!();
        macro_rules ! impl_chain_for_tuple { ($ mod_name : ident $ StructName : ident $ ($ F : ident) +) => { mod $ mod_name { # [repr (usize)] enum Indexes { $ ($ F ,) + } $ (pub (super) const $ F : usize = Indexes ::$ F as usize ;) + pub (super) const LEN : usize = [$ (Indexes ::$ F ,) +] . len () ; } # [pin_project :: pin_project] pub struct $ StructName <$ ($ F ,) +> { index : usize , done : bool , $ (# [pin] $ F : $ F ,) + } impl < T , $ ($ F ,) +> Stream for $ StructName <$ ($ F ,) +> where $ ($ F : Stream < Item = T >,) + { type Item = T ; fn poll_next (self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < Option < Self :: Item >> { let mut this = self . project () ; assert ! (!* this . done , "Stream should not be polled after completion") ; loop { if * this . index == $ mod_name :: LEN { * this . done = true ; return Poll :: Ready (None) ; } match * this . index { $ ($ mod_name ::$ F => { let fut = unsafe { Pin :: new_unchecked (& mut this .$ F) } ; match fut . poll_next (cx) { Poll :: Ready (None) => { * this . index += 1 ; continue ; } v @ (Poll :: Pending | Poll :: Ready (Some (_))) => return v , } } ,) + _ => unreachable ! () , } } } } impl <$ ($ F ,) +> fmt :: Debug for $ StructName <$ ($ F ,) +> where $ ($ F : fmt :: Debug ,) + { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . debug_tuple ("Chain") $ (. field (& self .$ F)) + . finish () } } impl < T , $ ($ F ,) +> Chain for ($ ($ F ,) +) where $ ($ F : Stream < Item = T >,) + { type Item = T ; type Stream = $ StructName <$ ($ F ,) +>; fn chain (self) -> Self :: Stream { let ($ ($ F ,) *) : ($ ($ F ,) *) = self ; Self :: Stream { done : false , index : 0 , $ ($ F ,) + } } } } }
    };
}

impl_chain_for_tuple!()