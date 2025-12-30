// Generated macro for error (module)
macro_rules! Depcrate_store_impls_dynamic_writeerror {
() => {
// Module: crate::store_impls::dynamic::write
// Provides: {"error"}
// Dependencies: {}
mod error { use crate :: { loose , store } ; # [doc = " The error returned by the [dynamic Store's][crate::Store] [`Write`](gix_object::Write) implementation."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] LoadIndex (# [from] store :: load_index :: Error) , # [error (transparent)] LooseWrite (# [from] loose :: write :: Error) , # [error (transparent)] Io (# [from] std :: io :: Error) , } }
};
}
