// Generated macro for alias_map (function)
macro_rules! Depcrate_writealias_map {
() => {
// Module: crate::write
// Provides: {"alias_map"}
// Dependencies: {}
# [doc = " Create a reverse-alias map from a value to all aliases having that value as a direct target"] fn alias_map (func : & Function) -> SecondaryMap < Value , Vec < Value > > { let mut aliases = SecondaryMap :: < _ , Vec < _ > > :: new () ; for v in func . dfg . values () { if let Some (k) = func . dfg . value_alias_dest_for_serialization (v) { aliases [k] . push (v) ; } } aliases }
};
}
