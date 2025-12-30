// Generated macro for Head (struct)
macro_rules! Depcrate_typesHead {
() => {
// Module: crate::types
// Provides: {"Head"}
// Dependencies: {}
# [doc = " The head reference, as created from looking at `.git/HEAD`, able to represent all of its possible states."] # [doc = ""] # [doc = " Note that like [`Reference`], this type's data is snapshot of persisted state on disk."] # [derive (Clone)] pub struct Head < 'repo > { # [doc = " One of various possible states for the HEAD reference"] pub kind : head :: Kind , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
