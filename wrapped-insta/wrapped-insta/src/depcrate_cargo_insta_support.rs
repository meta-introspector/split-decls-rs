// Generated macro for _cargo_insta_support (module)
macro_rules! Depcrate_cargo_insta_support {
() => {
// Module: crate
// Provides: {"_cargo_insta_support"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "_cargo_insta_internal")] pub mod _cargo_insta_support { pub use crate :: { content :: Error as ContentError , env :: { Error as ToolConfigError , OutputBehavior , SnapshotUpdate , TestRunner , ToolConfig , UnreferencedSnapshots , } , output :: SnapshotPrinter , snapshot :: PendingInlineSnapshot , snapshot :: SnapshotContents , snapshot :: TextSnapshotContents , utils :: get_cargo , utils :: is_ci , } ; }
};
}
