// Generated macro for Context (struct)
macro_rules! Depcrate_driver_applyContext {
() => {
// Module: crate::driver::apply
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Additional information for use in the [`State::apply()`] method."] # [derive (Debug , Copy , Clone)] pub struct Context < 'a , 'b > { # [doc = " The repo-relative using slashes as separator of the entry currently being processed."] pub rela_path : & 'a BStr , # [doc = " The name of the reference that `HEAD` is pointing to. It's passed to `process` filters if present."] pub ref_name : Option < & 'b BStr > , # [doc = " The root-level tree that contains the current entry directly or indirectly, or the commit owning the tree (if available)."] # [doc = ""] # [doc = " This is passed to `process` filters if present."] pub treeish : Option < gix_hash :: ObjectId > , # [doc = " The actual blob-hash of the data we are processing. It's passed to `process` filters if present."] # [doc = ""] # [doc = " Note that this hash might be different from the `$Id$` of the respective `ident` filter, as the latter generates the hash itself."] pub blob : Option < gix_hash :: ObjectId > , }
};
}
