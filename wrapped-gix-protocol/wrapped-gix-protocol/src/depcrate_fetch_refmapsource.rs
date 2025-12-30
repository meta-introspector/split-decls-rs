// Generated macro for Source (enum)
macro_rules! Depcrate_fetch_refmapSource {
() => {
// Module: crate::fetch::refmap
// Provides: {"Source"}
// Dependencies: {}
# [doc = " Either an object id that the remote has or the matched remote ref itself."] # [derive (Debug , Clone)] pub enum Source { # [doc = " An object id, as the matched ref-spec was an object id itself."] ObjectId (gix_hash :: ObjectId) , # [doc = " The remote reference that matched the ref-specs name."] Ref (crate :: handshake :: Ref) , }
};
}
