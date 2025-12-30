// Generated macro for InterfaceGenerator (struct)
macro_rules! Depcrate_interfaceInterfaceGenerator {
() => {
// Module: crate::interface
// Provides: {"InterfaceGenerator"}
// Dependencies: {}
pub struct InterfaceGenerator < 'a > { pub src : Source , pub (super) identifier : Identifier < 'a > , pub in_import : bool , pub sizes : SizeAlign , pub (super) r#gen : & 'a mut RustWasm , pub wasm_import_module : & 'a str , pub resolve : & 'a Resolve , pub return_pointer_area_size : ArchitectureSize , pub return_pointer_area_align : Alignment , pub (super) needs_runtime_module : bool , }
};
}
