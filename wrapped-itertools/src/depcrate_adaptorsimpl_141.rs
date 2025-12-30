// Generated macro for impl_141 (impl)
macro_rules! Depcrate_adaptorsimpl_141 {
() => {
// Module: crate::adaptors
// Provides: {"impl_141"}
// Dependencies: {}
impl < I , A > Iterator for WhileSome < I > where I : Iterator < Item = Option < A > > , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { match self . iter . next () { None | Some (None) => None , Some (elt) => elt , } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < B , F > (mut self , acc : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let res = self . iter . try_fold (acc , | acc , item | match item { Some (item) => Ok (f (acc , item)) , None => Err (acc) , }) ; match res { Ok (val) => val , Err (val) => val , } } }
};
}
