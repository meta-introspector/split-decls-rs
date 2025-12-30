// Generated macro for Names (type)
macro_rules! Depcrate_remoteNames {
() => {
// Module: crate::remote
// Provides: {"Names"}
// Dependencies: {}
# [doc = " A type-definition for a sorted list of unvalidated remote names - they have been read straight from the configuration."] pub type Names < 'a > = BTreeSet < Cow < 'a , BStr > > ;
};
}
