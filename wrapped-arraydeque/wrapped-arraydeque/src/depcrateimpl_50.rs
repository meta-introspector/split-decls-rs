// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < T , const CAP : usize > From < ArrayDeque < T , CAP , Wrapping > > for ArrayDeque < T , CAP , Saturating > { fn from (buf : ArrayDeque < T , CAP , Wrapping >) -> Self { buf . into_iter () . collect () } }
};
}
