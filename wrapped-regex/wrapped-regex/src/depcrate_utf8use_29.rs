// Generated macro for use_29 (use)
macro_rules! Depcrate_utf8use_29 {
() => {
// Module: crate::utf8
// Provides: {"use_29"}
// Dependencies: {}
# [doc = " A few elementary UTF-8 encoding and decoding functions used by the matching"] # [doc = " engines."] # [doc = ""] # [doc = " In an ideal world, the matching engines operate on `&str` and we can just"] # [doc = " lean on the standard library for all our UTF-8 needs. However, to support"] # [doc = " byte based regexes (that can match on arbitrary bytes which may contain"] # [doc = " UTF-8), we need to be capable of searching and decoding UTF-8 on a `&[u8]`."] # [doc = " The standard library doesn't really recognize this use case, so we have"] # [doc = " to build it out ourselves."] # [doc = ""] # [doc = " Should this be factored out into a separate crate? It seems independently"] # [doc = " useful. There are other crates that already exist (e.g., `utf-8`) that have"] # [doc = " overlapping use cases. Not sure what to do."] use std :: char ;
};
}
