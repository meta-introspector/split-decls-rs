// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl < T , const CAP : usize > From < ArrayDeque < T , CAP , Saturating > > for ArrayDeque < T , CAP , Wrapping > { fn from (buf : ArrayDeque < T , CAP , Saturating >) -> Self { buf . into_iter () . collect () } }
};
}
