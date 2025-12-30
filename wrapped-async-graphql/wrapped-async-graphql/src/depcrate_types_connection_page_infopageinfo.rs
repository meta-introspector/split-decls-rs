// Generated macro for PageInfo (struct)
macro_rules! Depcrate_types_connection_page_infoPageInfo {
() => {
// Module: crate::types::connection::page_info
// Provides: {"PageInfo"}
// Dependencies: {}
# [doc = " Information about pagination in a connection"] # [derive (SimpleObject)] # [graphql (internal , shareable)] pub struct PageInfo { # [doc = " When paginating backwards, are there more items?"] pub has_previous_page : bool , # [doc = " When paginating forwards, are there more items?"] pub has_next_page : bool , # [doc = " When paginating backwards, the cursor to continue."] pub start_cursor : Option < String > , # [doc = " When paginating forwards, the cursor to continue."] pub end_cursor : Option < String > , }
};
}
