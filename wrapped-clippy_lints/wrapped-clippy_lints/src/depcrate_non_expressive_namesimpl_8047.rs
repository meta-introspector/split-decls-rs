// Generated macro for impl_8047 (impl)
macro_rules! Depcrate_non_expressive_namesimpl_8047 {
() => {
// Module: crate::non_expressive_names
// Provides: {"impl_8047"}
// Dependencies: {}
impl SimilarNamesLocalVisitor < '_ , '_ > { # [doc = " ensure scoping rules work"] fn apply < F : for < 'c > Fn (& 'c mut Self) > (& mut self , f : F) { let n = self . names . len () ; let single_char_count = self . single_char_names . len () ; f (self) ; self . names . truncate (n) ; self . single_char_names . truncate (single_char_count) ; } }
};
}
