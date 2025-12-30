// Generated macro for impl_537 (impl)
macro_rules! Depcrate_stats_bivariateimpl_537 {
() => {
// Module: crate::stats::bivariate
// Provides: {"impl_537"}
// Dependencies: {}
impl < 'a , X , Y > Iterator for Pairs < 'a , X , Y > { type Item = (& 'a X , & 'a Y) ; fn next (& mut self) -> Option < (& 'a X , & 'a Y) > { if self . state < self . data . len () { let i = self . state ; self . state += 1 ; debug_assert ! (i < self . data . 0 . len ()) ; debug_assert ! (i < self . data . 1 . len ()) ; unsafe { Some ((self . data . 0 . get_unchecked (i) , self . data . 1 . get_unchecked (i))) } } else { None } } }
};
}
