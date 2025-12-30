// Generated macro for impl_73 (impl)
macro_rules! Depcrate_tableimpl_73 {
() => {
// Module: crate::table
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'line > fmt :: Display for Error < 'line > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: SurpriseContinuationLine => { write ! (f , "continuation line follows line that isn't a zone definition line") } Error :: UnknownRuleset (_) => { write ! (f , "zone definition refers to a ruleset that isn't defined") } Error :: DuplicateLink (_) => write ! (f , "link line with name that already exists") , Error :: DuplicateZone => write ! (f , "zone line with name that already exists") , } } }
};
}
