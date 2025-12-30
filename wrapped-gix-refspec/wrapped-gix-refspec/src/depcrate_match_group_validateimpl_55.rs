// Generated macro for impl_55 (impl)
macro_rules! Depcrate_match_group_validateimpl_55 {
() => {
// Module: crate::match_group::validate
// Provides: {"impl_55"}
// Dependencies: {}
impl std :: fmt :: Display for Issue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Issue :: Conflict { destination_full_ref_name , sources , specs , } => { write ! (f , "Conflicting destination {destination_full_ref_name:?} would be written by {}" , sources . iter () . zip (specs . iter ()) . map (| (src , spec) | format ! ("{src} ({spec:?})")) . collect ::< Vec < _ >> () . join (", ")) } } } }
};
}
