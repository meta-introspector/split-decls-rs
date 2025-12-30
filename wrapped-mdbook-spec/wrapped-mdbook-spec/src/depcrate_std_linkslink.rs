// Generated macro for Link (struct)
macro_rules! Depcrate_std_linksLink {
() => {
// Module: crate::std_links
// Provides: {"Link"}
// Dependencies: {}
# [derive (Debug)] struct Link < 'a > { link_type : LinkType , # [doc = " Where the link is going to, for example `std::ffi::OsString`."] dest_url : CowStr < 'a > , # [doc = " The span in the original markdown where the link is located."] # [doc = ""] # [doc = " Note that this is the post-processed markdown (such as having rules"] # [doc = " expanded), not the markdown on the disk."] # [doc = ""] # [doc = " Note that during translation, all links will be converted to inline"] # [doc = " links. That means that for reference-style links, the link reference"] # [doc = " definition will end up being ignored in the final markdown. For"] # [doc = " example, a link like ``[`OsString`]`` with a definition"] # [doc = " ``[`OsString`]: std::ffi::OsString`` will convert the link to"] # [doc = " ``[`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html)`."] range : Range < usize > , }
};
}
