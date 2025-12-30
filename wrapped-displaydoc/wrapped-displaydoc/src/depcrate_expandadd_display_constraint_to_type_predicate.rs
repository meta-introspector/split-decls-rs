// Generated macro for add_display_constraint_to_type_predicate (function)
macro_rules! Depcrate_expandadd_display_constraint_to_type_predicate {
() => {
// Module: crate::expand
// Provides: {"add_display_constraint_to_type_predicate"}
// Dependencies: {}
# [doc = " Add a requirement for [core::fmt::Display] to a `where` predicate for some type."] fn add_display_constraint_to_type_predicate (predicate_that_needs_a_display_impl : & mut PredicateType ,) { let display_path = join_paths (& ["core" , "fmt" , "Display"] , UseGlobalPrefix :: LeadingColon) ; let display_bound = TypeParamBound :: Trait (TraitBound { paren_token : None , modifier : TraitBoundModifier :: None , lifetimes : None , path : display_path , }) ; if ! predicate_that_needs_a_display_impl . bounds . is_empty () { predicate_that_needs_a_display_impl . bounds . push_punct (Plus { spans : [Span :: call_site ()] , }) ; } predicate_that_needs_a_display_impl . bounds . push_value (display_bound) ; }
};
}
