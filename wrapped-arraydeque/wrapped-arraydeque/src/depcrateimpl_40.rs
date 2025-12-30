// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < T , const CAP : usize > FromIterator < T > for ArrayDeque < T , CAP , Saturating > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut array : ArrayDeque < _ , CAP , Saturating > = ArrayDeque :: new () ; array . extend_back (iter) ; array } }
};
}
