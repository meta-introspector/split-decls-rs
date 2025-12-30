// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < T : FloatCore + Signed > Signed for OrderedFloat < T > { # [inline] fn abs (& self) -> Self { OrderedFloat (self . 0 . abs ()) } fn abs_sub (& self , other : & Self) -> Self { OrderedFloat (Signed :: abs_sub (& self . 0 , & other . 0)) } # [inline] fn signum (& self) -> Self { OrderedFloat (self . 0 . signum ()) } # [inline] fn is_positive (& self) -> bool { self . 0 . is_positive () } # [inline] fn is_negative (& self) -> bool { self . 0 . is_negative () } }
};
}
