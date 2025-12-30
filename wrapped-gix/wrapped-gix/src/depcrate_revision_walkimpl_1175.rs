// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_revision_walkimpl_1175 {
() => {
// Module: crate::revision::walk
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'repo > Platform < 'repo > { pub (crate) fn new (tips : impl IntoIterator < Item = impl Into < ObjectId > > , repo : & 'repo Repository) -> Self { revision :: walk :: Platform { repo , tips : tips . into_iter () . map (Into :: into) . collect () , hidden : Vec :: new () , sorting : Default :: default () , parents : Default :: default () , use_commit_graph : None , commit_graph : None , boundary : Vec :: new () , } } }
};
}
