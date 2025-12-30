// Generated macro for impl_202 (impl)
macro_rules! Depcrateimpl_202 {
() => {
// Module: crate
// Provides: {"impl_202"}
// Dependencies: {}
impl < T : FloatCore + Signed > Signed for NotNan < T > { # [inline] fn abs (& self) -> Self { NotNan (self . 0 . abs ()) } fn abs_sub (& self , other : & Self) -> Self { NotNan :: new (Signed :: abs_sub (& self . 0 , & other . 0)) . expect ("Subtraction resulted in NaN") } # [inline] fn signum (& self) -> Self { NotNan (self . 0 . signum ()) } # [inline] fn is_positive (& self) -> bool { self . 0 . is_positive () } # [inline] fn is_negative (& self) -> bool { self . 0 . is_negative () } }
};
}
