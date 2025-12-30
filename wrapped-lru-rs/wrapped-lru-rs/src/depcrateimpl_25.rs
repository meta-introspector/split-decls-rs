// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < K , Q > Borrow < KeyWrapper < Q > > for KeyRef < K > where K : Borrow < Q > , Q : ? Sized , { fn borrow (& self) -> & KeyWrapper < Q > { let key = unsafe { & * self . k } . borrow () ; KeyWrapper :: from_ref (key) } }
};
}
