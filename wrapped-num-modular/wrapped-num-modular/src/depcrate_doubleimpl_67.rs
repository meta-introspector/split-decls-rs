// Generated macro for impl_67 (impl)
macro_rules! Depcrate_doubleimpl_67 {
() => {
// Module: crate::double
// Provides: {"impl_67"}
// Dependencies: {}
impl Div < umax > for udouble { type Output = Self ; # [inline] fn div (self , rhs : umax) -> Self :: Output { if self . hi < rhs { Self { lo : self . div_rem_2by1 (rhs) . 0 , hi : 0 , } } else { let (q , r) = div_rem (self . hi , rhs) ; Self { lo : Self { lo : self . lo , hi : r } . div_rem_2by1 (rhs) . 0 , hi : q , } } } }
};
}
