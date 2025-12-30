// Generated macro for ComdatKind (enum)
macro_rules! Depcrate_commonComdatKind {
() => {
// Module: crate::common
// Provides: {"ComdatKind"}
// Dependencies: {}
# [doc = " The selection kind for a COMDAT section group."] # [doc = ""] # [doc = " This determines the way in which the linker resolves multiple definitions of the COMDAT"] # [doc = " sections."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum ComdatKind { # [doc = " The selection kind is unknown."] Unknown , # [doc = " Multiple definitions are allowed."] # [doc = ""] # [doc = " An arbitrary definition is selected, and the rest are removed."] # [doc = ""] # [doc = " This is the only supported selection kind for ELF."] Any , # [doc = " Multiple definitions are not allowed."] # [doc = ""] # [doc = " This is used to group sections without allowing duplicates."] NoDuplicates , # [doc = " Multiple definitions must have the same size."] # [doc = ""] # [doc = " An arbitrary definition is selected, and the rest are removed."] SameSize , # [doc = " Multiple definitions must match exactly."] # [doc = ""] # [doc = " An arbitrary definition is selected, and the rest are removed."] ExactMatch , # [doc = " Multiple definitions are allowed, and the largest is selected."] # [doc = ""] # [doc = " An arbitrary definition with the largest size is selected, and the rest are removed."] Largest , # [doc = " Multiple definitions are allowed, and the newest is selected."] Newest , }
};
}
