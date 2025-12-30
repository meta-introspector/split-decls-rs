// Generated macro for impl_49 (impl)
macro_rules! Depcrate_patternimpl_49 {
() => {
// Module: crate::pattern
// Provides: {"impl_49"}
// Dependencies: {}
impl std :: fmt :: Display for InvalidPatternError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "found invalid UTF-8 in pattern at byte offset {}: {} \
             (disable Unicode mode and use hex escape sequences to match \
             arbitrary bytes in a pattern, e.g., '(?-u)\\xFF')" , self . valid_up_to , self . original ,) } }
};
}
