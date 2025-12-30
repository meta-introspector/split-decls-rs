// Generated macro for ItemIdentifier (struct)
macro_rules! Depcrate_idItemIdentifier {
() => {
// Module: crate::id
// Provides: {"ItemIdentifier"}
// Dependencies: {}
# [doc = " Names in C and Objective-C are global, so this is always enough to"] # [doc = " uniquely identify an item."] # [doc = ""] # [doc = " Often, though, we want to know the library, file name and general location"] # [doc = " an item came from as well."] # [derive (Debug , Clone)] pub struct ItemIdentifier < N = String > { # [doc = " The name of the item in Rust (i.e. it may have been renamed)."] pub name : N , location : Location , }
};
}
