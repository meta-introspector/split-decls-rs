// Generated macro for SimilarNamesLocalVisitor (struct)
macro_rules! Depcrate_non_expressive_namesSimilarNamesLocalVisitor {
() => {
// Module: crate::non_expressive_names
// Provides: {"SimilarNamesLocalVisitor"}
// Dependencies: {}
struct SimilarNamesLocalVisitor < 'a , 'tcx > { names : Vec < ExistingName > , cx : & 'a EarlyContext < 'tcx > , threshold : u64 , # [doc = " A stack of scopes containing the single-character bindings in each scope."] single_char_names : Vec < Vec < Ident > > , }
};
}
