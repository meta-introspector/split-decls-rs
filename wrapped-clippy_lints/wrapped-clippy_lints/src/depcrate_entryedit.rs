// Generated macro for Edit (enum)
macro_rules! Depcrate_entryEdit {
() => {
// Module: crate::entry
// Provides: {"Edit"}
// Dependencies: {}
# [doc = " An edit that will need to be made to move the expression to use the entry api"] # [derive (Clone , Copy)] enum Edit < 'tcx > { # [doc = " A semicolon that needs to be removed. Used to create a closure for `insert_with`."] RemoveSemi (Span) , # [doc = " An insertion into the map."] Insertion (Insertion < 'tcx >) , }
};
}
