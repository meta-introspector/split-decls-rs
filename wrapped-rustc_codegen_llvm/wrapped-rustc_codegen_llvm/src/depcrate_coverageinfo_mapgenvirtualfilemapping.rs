// Generated macro for VirtualFileMapping (struct)
macro_rules! Depcrate_coverageinfo_mapgenVirtualFileMapping {
() => {
// Module: crate::coverageinfo::mapgen
// Provides: {"VirtualFileMapping"}
// Dependencies: {}
# [doc = " Holds a mapping from \"local\" (per-function) file IDs to their corresponding"] # [doc = " source files."] # [derive (Debug , Default)] struct VirtualFileMapping { local_file_table : IndexVec < LocalFileId , Arc < SourceFile > > , }
};
}
