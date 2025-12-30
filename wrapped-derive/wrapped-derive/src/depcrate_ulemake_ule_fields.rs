// Generated macro for make_ule_fields (function)
macro_rules! Depcrate_ulemake_ule_fields {
() => {
// Module: crate::ule
// Provides: {"make_ule_fields"}
// Dependencies: {}
# [doc = " Make corresponding ULE fields for each field"] pub (crate) fn make_ule_fields (fields : & [FieldInfo]) -> Vec < TokenStream2 > { fields . iter () . map (| f | { let ty = & f . field . ty ; let ty = quote ! (<# ty as zerovec :: ule :: AsULE >:: ULE) ; let setter = f . setter () ; let vis = & f . field . vis ; quote ! (# vis # setter # ty) }) . collect :: < Vec < _ > > () }
};
}
