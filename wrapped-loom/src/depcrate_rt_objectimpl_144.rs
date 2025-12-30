// Generated macro for impl_144 (impl)
macro_rules! Depcrate_rt_objectimpl_144 {
() => {
// Module: crate::rt::object
// Provides: {"impl_144"}
// Dependencies: {}
impl < T > Ref < T > { # [doc = " Erase the type marker"] pub (super) fn erase (self) -> Ref < () > { Ref { index : self . index , _p : PhantomData , } } pub (super) fn ref_eq (self , other : Ref < T >) -> bool { self . index == other . index } }
};
}
