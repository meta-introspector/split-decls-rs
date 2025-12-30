// Generated macro for impl_226 (impl)
macro_rules! Depcrate_pack_receiveimpl_226 {
() => {
// Module: crate::pack::receive
// Provides: {"impl_226"}
// Dependencies: {}
impl JsonOutcome { pub fn from_outcome_and_refs (v : pack :: bundle :: write :: Outcome , refs : & [Ref]) -> Self { JsonOutcome { index : v . index . into () , pack_kind : v . pack_version , index_path : v . index_path , data_path : v . data_path , refs : refs . iter () . cloned () . map (Into :: into) . collect () , } } }
};
}
