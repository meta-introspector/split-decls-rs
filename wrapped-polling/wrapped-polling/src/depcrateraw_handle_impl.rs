// Generated macro for raw_handle_impl (module)
macro_rules! Depcrateraw_handle_impl {
() => {
// Module: crate
// Provides: {"raw_handle_impl"}
// Dependencies: {}
# [cfg (windows)] # [cfg_attr (docsrs , doc (cfg (windows)))] mod raw_handle_impl { use crate :: Poller ; use std :: os :: windows :: io :: { AsHandle , AsRawHandle , BorrowedHandle , RawHandle } ; impl AsRawHandle for Poller { fn as_raw_handle (& self) -> RawHandle { self . poller . as_raw_handle () } } impl AsHandle for Poller { fn as_handle (& self) -> BorrowedHandle < '_ > { self . poller . as_handle () } } }
};
}
