// Generated macro for impl_77 (impl)
macro_rules! Depcrate_rcvecimpl_77 {
() => {
// Module: crate::rcvec
// Provides: {"impl_77"}
// Dependencies: {}
impl < T > RcVec < T > { pub (crate) fn is_empty (& self) -> bool { self . inner . is_empty () } pub (crate) fn len (& self) -> usize { self . inner . len () } pub (crate) fn iter (& self) -> slice :: Iter < T > { self . inner . iter () } pub (crate) fn make_mut (& mut self) -> RcVecMut < T > where T : Clone , { RcVecMut { inner : Rc :: make_mut (& mut self . inner) , } } pub (crate) fn get_mut (& mut self) -> Option < RcVecMut < T > > { let inner = Rc :: get_mut (& mut self . inner) ? ; Some (RcVecMut { inner }) } pub (crate) fn make_owned (mut self) -> RcVecBuilder < T > where T : Clone , { let vec = if let Some (owned) = Rc :: get_mut (& mut self . inner) { mem :: take (owned) } else { Vec :: clone (& self . inner) } ; RcVecBuilder { inner : vec } } }
};
}
