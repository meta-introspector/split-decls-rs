// Generated macro for impl_294 (impl)
macro_rules! Depcrate_sourceimpl_294 {
() => {
// Module: crate::source
// Provides: {"impl_294"}
// Dependencies: {}
impl Source for Vec < Box < dyn Source + Send + Sync > > { fn clone_into_box (& self) -> Box < dyn Source + Send + Sync > { Box :: new ((* self) . clone ()) } fn collect (& self) -> Result < Map < String , Value > > { let mut cache : Value = Map :: < String , Value > :: new () . into () ; for source in self { source . collect_to (& mut cache) ? ; } if let ValueKind :: Table (table) = cache . kind { Ok (table) } else { unreachable ! () ; } } }
};
}
