// Generated macro for impl_211 (impl)
macro_rules! Depcrate_out_refimpl_211 {
() => {
// Module: crate::out_ref
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'a , T : Type < T > > From < & 'a mut T :: Default > for OutRef < 'a , T > { fn from (from : & 'a mut T :: Default) -> Self { unsafe { core :: mem :: transmute (from) } } }
};
}
