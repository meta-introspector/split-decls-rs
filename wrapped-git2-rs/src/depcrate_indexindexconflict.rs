// Generated macro for IndexConflict (struct)
macro_rules! Depcrate_indexIndexConflict {
() => {
// Module: crate::index
// Provides: {"IndexConflict"}
// Dependencies: {}
# [doc = " A structure to represent the information returned when a conflict is detected in an index entry"] pub struct IndexConflict { # [doc = " The ancestor index entry of the two conflicting index entries"] pub ancestor : Option < IndexEntry > , # [doc = " The index entry originating from the user's copy of the repository."] # [doc = " Its contents conflict with 'their' index entry"] pub our : Option < IndexEntry > , # [doc = " The index entry originating from the external repository."] # [doc = " Its contents conflict with 'our' index entry"] pub their : Option < IndexEntry > , }
};
}
