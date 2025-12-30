// Generated macro for IndexerCallbacks (struct)
macro_rules! DepcrateIndexerCallbacks {
() => {
// Module: crate
// Provides: {"IndexerCallbacks"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] # [rustfmt :: skip] pub struct IndexerCallbacks { pub abortQuery : Option < extern "C" fn (CXClientData , * mut c_void) -> c_int > , pub diagnostic : Option < extern "C" fn (CXClientData , CXDiagnosticSet , * mut c_void) > , pub enteredMainFile : Option < extern "C" fn (CXClientData , CXFile , * mut c_void) -> CXIdxClientFile > , pub ppIncludedFile : Option < extern "C" fn (CXClientData , * const CXIdxIncludedFileInfo) -> CXIdxClientFile > , pub importedASTFile : Option < extern "C" fn (CXClientData , * const CXIdxImportedASTFileInfo) -> CXIdxClientASTFile > , pub startedTranslationUnit : Option < extern "C" fn (CXClientData , * mut c_void) -> CXIdxClientContainer > , pub indexDeclaration : Option < extern "C" fn (CXClientData , * const CXIdxDeclInfo) > , pub indexEntityReference : Option < extern "C" fn (CXClientData , * const CXIdxEntityRefInfo) > , }
};
}
