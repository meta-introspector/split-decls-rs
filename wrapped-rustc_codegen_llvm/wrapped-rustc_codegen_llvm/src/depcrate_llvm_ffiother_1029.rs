// Generated macro for other_1029 (other)
macro_rules! Depcrate_llvm_ffiother_1029 {
() => {
// Module: crate::llvm::ffi
// Provides: {"other_1029"}
// Dependencies: {}
unsafe extern "C" { pub (crate) fn LLVMCreateDIBuilder < 'll > (M : & 'll Module) -> * mut DIBuilder < 'll > ; pub (crate) fn LLVMDisposeDIBuilder < 'll > (Builder : ptr :: NonNull < DIBuilder < 'll > >) ; pub (crate) fn LLVMDIBuilderFinalize < 'll > (Builder : & DIBuilder < 'll >) ; pub (crate) fn LLVMDIBuilderCreateNameSpace < 'll > (Builder : & DIBuilder < 'll > , ParentScope : Option < & 'll Metadata > , Name : * const c_uchar , NameLen : size_t , ExportSymbols : llvm :: Bool ,) -> & 'll Metadata ; pub (crate) fn LLVMDIBuilderCreateLexicalBlock < 'll > (Builder : & DIBuilder < 'll > , Scope : & 'll Metadata , File : & 'll Metadata , Line : c_uint , Column : c_uint ,) -> & 'll Metadata ; pub (crate) fn LLVMDIBuilderCreateLexicalBlockFile < 'll > (Builder : & DIBuilder < 'll > , Scope : & 'll Metadata , File : & 'll Metadata , Discriminator : c_uint ,) -> & 'll Metadata ; pub (crate) fn LLVMDIBuilderCreateDebugLocation < 'll > (Ctx : & 'll Context , Line : c_uint , Column : c_uint , Scope : & 'll Metadata , InlinedAt : Option < & 'll Metadata > ,) -> & 'll Metadata ; }
};
}
