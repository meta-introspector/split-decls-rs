// Generated macro for impl_12 (impl)
macro_rules! Depcrate_patternimpl_12 {
() => {
// Module: crate::pattern
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Display for Pattern { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . mode . contains (Mode :: NEGATIVE) { "!" . fmt (f) ? ; } if self . mode . contains (Mode :: ABSOLUTE) { "/" . fmt (f) ? ; } self . text . fmt (f) ? ; if self . mode . contains (Mode :: MUST_BE_DIR) { "/" . fmt (f) ? ; } Ok (()) } }
};
}
