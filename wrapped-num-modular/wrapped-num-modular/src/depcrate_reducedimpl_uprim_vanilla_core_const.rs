// Generated macro for impl_uprim_vanilla_core_const (macro)
macro_rules! Depcrate_reducedimpl_uprim_vanilla_core_const {
() => {
// Module: crate::reduced
// Provides: {"impl_uprim_vanilla_core_const"}
// Dependencies: {}
macro_rules ! impl_uprim_vanilla_core_const { ($ ($ T : ty) *) => { $ (impl Vanilla <$ T > { # [inline] pub (crate) const fn add (m : &$ T , lhs : $ T , rhs : $ T) -> $ T { let (sum , overflow) = lhs . overflowing_add (rhs) ; if overflow || sum >= * m { let (sum2 , overflow2) = sum . overflowing_sub (* m) ; debug_assert ! (overflow == overflow2) ; sum2 } else { sum } } # [inline] pub (crate) const fn dbl (m : &$ T , target : $ T) -> $ T { Self :: add (m , target , target) } # [inline] pub (crate) const fn sub (m : &$ T , lhs : $ T , rhs : $ T) -> $ T { if lhs >= rhs { lhs - rhs } else { * m - (rhs - lhs) } } # [inline] pub (crate) const fn neg (m : &$ T , target : $ T) -> $ T { match target { 0 => 0 , x => * m - x } } }) * } ; }
};
}
