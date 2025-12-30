// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , const CAP : usize > FromIterator < T > for ArrayDeque < T , CAP , Wrapping > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut array : ArrayDeque < _ , CAP , Wrapping > = ArrayDeque :: new () ; array . extend_back (iter) ; array } }
};
}
