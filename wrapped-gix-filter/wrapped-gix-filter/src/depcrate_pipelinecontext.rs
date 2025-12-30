// Generated macro for Context (struct)
macro_rules! Depcrate_pipelineContext {
() => {
// Module: crate::pipeline
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Context that typically doesn't change throughout the lifetime of a pipeline, for use with `process` filters."] # [doc = ""] # [doc = " Note that this is quite specific to third-party filters that actually make use of this additional context."] # [derive (Default , Debug , Clone)] pub struct Context { # [doc = " The name of the reference that `HEAD` is pointing to. It's passed to `process` filters if present."] pub ref_name : Option < BString > , # [doc = " The root-level tree that contains the current entry directly or indirectly, or the commit owning the tree (if available)."] # [doc = ""] # [doc = " This is passed to `process` filters if present."] pub treeish : Option < gix_hash :: ObjectId > , # [doc = " The actual blob-hash of the data we are processing. It's passed to `process` filters if present."] # [doc = ""] # [doc = " Note that this hash might be different from the `$Id$` of the respective `ident` filter, as the latter generates the hash itself."] pub blob : Option < gix_hash :: ObjectId > , }
};
}
