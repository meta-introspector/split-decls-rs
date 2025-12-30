// Generated macro for RootQueryDb (trait)
macro_rules! DepcrateRootQueryDb {
() => {
// Module: crate
// Provides: {"RootQueryDb"}
// Dependencies: {}
# [doc = " Database which stores all significant input facts: source code and project"] # [doc = " model. Everything else in rust-analyzer is derived from these queries."] # [query_group :: query_group] pub trait RootQueryDb : SourceDatabase + salsa :: Database { # [doc = " Parses the file into the syntax tree."] # [salsa :: invoke (parse)] # [salsa :: lru (128)] fn parse (& self , file_id : EditionedFileId) -> Parse < ast :: SourceFile > ; # [doc = " Returns the set of errors obtained from parsing the file including validation errors."] # [salsa :: transparent] fn parse_errors (& self , file_id : EditionedFileId) -> Option < & [SyntaxError] > ; # [salsa :: transparent] fn toolchain_channel (& self , krate : Crate) -> Option < ReleaseChannel > ; # [doc = " Crates whose root file is in `id`."] # [salsa :: invoke_interned (source_root_crates)] fn source_root_crates (& self , id : SourceRootId) -> Arc < [Crate] > ; # [salsa :: transparent] fn relevant_crates (& self , file_id : FileId) -> Arc < [Crate] > ; # [doc = " Returns the crates in topological order."] # [doc = ""] # [doc = " **Warning**: do not use this query in `hir-*` crates! It kills incrementality across crate metadata modifications."] # [salsa :: input] fn all_crates (& self) -> Arc < Box < [Crate] > > ; }
};
}
