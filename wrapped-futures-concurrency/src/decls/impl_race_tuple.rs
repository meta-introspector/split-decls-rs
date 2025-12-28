macro_rules! deps {
    () => {
        Indexer!();
        Race!();
    };
}

macro_rules! impl_race_tuple {
    () => {
        deps!();
        macro_rules ! impl_race_tuple { ($ StructName : ident $ ($ F : ident) +) => { # [doc = " A future which waits for the first future to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`race`] method on the [`Race`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`race`]: crate::future::Race::race"] # [doc = " [`Race`]: crate::future::Race"] # [pin_project] # [must_use = "futures do nothing unless you `.await` or poll them"] # [allow (non_snake_case)] pub struct $ StructName < T , $ ($ F) ,*> where $ ($ F : Future < Output = T >,) * { done : bool , indexer : utils :: Indexer , $ (# [pin] $ F : $ F ,) * } impl < T , $ ($ F) ,*> Debug for $ StructName < T , $ ($ F) ,*> where $ ($ F : Future < Output = T > + Debug ,) * { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . debug_tuple ("Race") $ (. field (& self .$ F)) * . finish () } } impl < T , $ ($ F) ,*> RaceTrait for ($ ($ F ,) *) where $ ($ F : IntoFuture < Output = T >,) * { type Output = T ; type Future = $ StructName < T , $ ($ F :: IntoFuture) ,*>; fn race (self) -> Self :: Future { let ($ ($ F ,) *) : ($ ($ F ,) *) = self ; $ StructName { done : false , indexer : utils :: Indexer :: new (utils :: tuple_len ! ($ ($ F ,) *)) , $ ($ F : $ F . into_future ()) ,* } } } impl < T , $ ($ F : Future) ,*> Future for $ StructName < T , $ ($ F) ,*> where $ ($ F : Future < Output = T >) ,* { type Output = T ; fn poll (self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < Self :: Output > { let mut this = self . project () ; assert ! (!* this . done , "Futures must not be polled after completing") ; # [repr (usize)] enum Indexes { $ ($ F) ,* } for i in this . indexer . iter () { utils :: gen_conditions ! (i , this , cx , poll , $ ((Indexes ::$ F as usize ; $ F , { Poll :: Ready (output) => { * this . done = true ; return Poll :: Ready (output) ; } , _ => continue , })) *) ; } Poll :: Pending } } } ; }
    };
}

impl_race_tuple!();