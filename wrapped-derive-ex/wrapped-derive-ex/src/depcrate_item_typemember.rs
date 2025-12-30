// Generated macro for member (function)
macro_rules! Depcrate_item_typemember {
() => {
// Module: crate::item_type
// Provides: {"member"}
// Dependencies: {}
fn member (this : TokenStream , field : & FieldEntry) -> TokenStream { let member = field . member () ; quote ! (# this .# member) }
};
}
