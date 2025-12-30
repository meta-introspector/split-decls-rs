// Generated macro for FileChange (struct)
macro_rules! Depcrate_changeFileChange {
() => {
// Module: crate::change
// Provides: {"FileChange"}
// Dependencies: {}
# [doc = " Encapsulate a bunch of raw `.set` calls on the database."] # [derive (Default)] pub struct FileChange { pub roots : Option < Vec < SourceRoot > > , pub files_changed : Vec < (FileId , Option < String >) > , pub crate_graph : Option < CrateGraphBuilder > , }
};
}
