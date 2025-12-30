// Generated macro for Table (type)
macro_rules! Depcrate_bundleTable {
() => {
// Module: crate::bundle
// Provides: {"Table"}
// Dependencies: {}
# [doc = " A table of [`Resource`]s indexed by a string-based [`Key`]."] pub type Table < 'a > = BTreeMap < Key < 'a > , Resource < 'a > > ;
};
}
