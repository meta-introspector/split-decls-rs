// Generated macro for CompiledModule (struct)
macro_rules! DepcrateCompiledModule {
() => {
// Module: crate
// Provides: {"CompiledModule"}
// Dependencies: {}
# [derive (Debug , Encodable , Decodable)] pub struct CompiledModule { pub name : String , pub kind : ModuleKind , pub object : Option < PathBuf > , pub dwarf_object : Option < PathBuf > , pub bytecode : Option < PathBuf > , pub assembly : Option < PathBuf > , pub llvm_ir : Option < PathBuf > , pub links_from_incr_cache : Vec < PathBuf > , }
};
}
