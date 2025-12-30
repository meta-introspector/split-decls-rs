// Generated macro for Sorter (enum)
macro_rules! Depcrate_walkSorter {
() => {
// Module: crate::walk
// Provides: {"Sorter"}
// Dependencies: {}
# [derive (Clone)] enum Sorter { ByName (Arc < dyn Fn (& OsStr , & OsStr) -> Ordering + Send + Sync + 'static >) , ByPath (Arc < dyn Fn (& Path , & Path) -> Ordering + Send + Sync + 'static >) , }
};
}
