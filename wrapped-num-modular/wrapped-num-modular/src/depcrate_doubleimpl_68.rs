// Generated macro for impl_68 (impl)
macro_rules! Depcrate_doubleimpl_68 {
() => {
// Module: crate::double
// Provides: {"impl_68"}
// Dependencies: {}
impl Rem < umax > for udouble { type Output = umax ; # [inline] fn rem (self , rhs : umax) -> Self :: Output { if self . hi < rhs { self . div_rem_2by1 (rhs) . 1 } else { Self { lo : self . lo , hi : self . hi % rhs , } . div_rem_2by1 (rhs) . 1 } } }
};
}
