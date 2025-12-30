// Generated macro for module_needs_externref_metadata (function)
macro_rules! Depcrate_externrefmodule_needs_externref_metadata {
() => {
// Module: crate::externref
// Provides: {"module_needs_externref_metadata"}
// Dependencies: {}
# [doc = " This function shouldn't need to exist, see the fixme at the call-site."] fn module_needs_externref_metadata (aux : & WasmBindgenAux , section : & NonstandardWitSection) -> bool { use Instruction :: * ; if ! aux . imports_with_catch . is_empty () { return true ; } section . adapters . iter () . any (| (_ , adapter) | { let instructions = match & adapter . kind { AdapterKind :: Local { instructions } => instructions , AdapterKind :: Import { .. } => return false , } ; instructions . iter () . any (| instr | { matches ! (instr . instr , VectorToMemory { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | MutableSliceToMemory { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | OptionVector { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | VectorLoad { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | OptionVectorLoad { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | View { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. } | OptionView { kind : VectorKind :: Externref | VectorKind :: NamedExternref (_) , .. }) }) }) }
};
}
