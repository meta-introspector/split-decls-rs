// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > Iterator for Params < 'a > { type Item = (& 'a str , & 'a str) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . 0 { ParamsInner :: Utf8 => { let value = ("charset" , "utf-8") ; self . 0 = ParamsInner :: None ; Some (value) } , ParamsInner :: Inlined (source , ref mut inline) => { let next = match * inline { Inline :: Done => { None } Inline :: One (one) => { * inline = Inline :: Done ; Some (one) } , Inline :: Two (one , two) => { * inline = Inline :: One (two) ; Some (one) } , } ; next . map (| (name , value) | { let name = & source . as_ref () [range (name)] ; let value = & source . as_ref () [range (value)] ; (name , value) }) } , ParamsInner :: Custom { source , ref mut params } => { params . next () . map (| & (name , value) | { let name = & source . as_ref () [range (name)] ; let value = & source . as_ref () [range (value)] ; (name , value) }) } , ParamsInner :: None => None , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match self . 0 { ParamsInner :: Utf8 => (1 , Some (1)) , ParamsInner :: Inlined (_ , Inline :: Done) => (0 , Some (0)) , ParamsInner :: Inlined (_ , Inline :: One (..)) => (1 , Some (1)) , ParamsInner :: Inlined (_ , Inline :: Two (..)) => (2 , Some (2)) , ParamsInner :: Custom { ref params , .. } => params . size_hint () , ParamsInner :: None => (0 , Some (0)) , } } }
};
}
