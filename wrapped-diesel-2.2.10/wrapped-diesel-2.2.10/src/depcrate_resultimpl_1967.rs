// Generated macro for impl_1967 (impl)
macro_rules! Depcrate_resultimpl_1967 {
() => {
// Module: crate::result
// Provides: {"impl_1967"}
// Dependencies: {}
impl < T > OptionalEmptyChangesetExtension < T > for QueryResult < T > { fn optional_empty_changeset (self) -> Result < Option < T > , Error > { match self { Ok (value) => Ok (Some (value)) , Err (Error :: QueryBuilderError (e)) if e . is :: < EmptyChangeset > () => Ok (None) , Err (e) => Err (e) , } } }
};
}
