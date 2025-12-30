// Generated macro for impl_signed (macro)
macro_rules! Depcrate_num_traitsimpl_signed {
() => {
// Module: crate::num_traits
// Provides: {"impl_signed"}
// Dependencies: {}
macro_rules ! impl_signed { ($ ty : ty) => { impl :: num_traits :: Signed for $ ty { # [inline] fn abs (& self) -> Self { :: num_traits :: float :: Float :: abs (* self) } # [inline] fn abs_sub (& self , other : & Self) -> Self { :: num_traits :: float :: Float :: abs_sub (* self , * other) } # [inline] fn signum (& self) -> Self { :: num_traits :: float :: Float :: signum (* self) } # [inline] fn is_positive (& self) -> bool { :: num_traits :: float :: Float :: is_sign_positive (* self) } # [inline] fn is_negative (& self) -> bool { :: num_traits :: float :: Float :: is_sign_negative (* self) } } } ; }
};
}
