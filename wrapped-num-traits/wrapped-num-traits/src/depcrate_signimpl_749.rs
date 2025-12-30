// Generated macro for impl_749 (impl)
macro_rules! Depcrate_signimpl_749 {
() => {
// Module: crate::sign
// Provides: {"impl_749"}
// Dependencies: {}
impl < T : Signed > Signed for Wrapping < T > where Wrapping < T > : Num + Neg < Output = Wrapping < T > > , { # [inline] fn abs (& self) -> Self { Wrapping (self . 0 . abs ()) } # [inline] fn abs_sub (& self , other : & Self) -> Self { Wrapping (self . 0 . abs_sub (& other . 0)) } # [inline] fn signum (& self) -> Self { Wrapping (self . 0 . signum ()) } # [inline] fn is_positive (& self) -> bool { self . 0 . is_positive () } # [inline] fn is_negative (& self) -> bool { self . 0 . is_negative () } }
};
}
