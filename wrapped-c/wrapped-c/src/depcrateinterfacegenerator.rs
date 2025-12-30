// Generated macro for InterfaceGenerator (struct)
macro_rules! DepcrateInterfaceGenerator {
() => {
// Module: crate
// Provides: {"InterfaceGenerator"}
// Dependencies: {}
struct InterfaceGenerator < 'a > { src : Source , in_import : bool , r#gen : & 'a mut C , resolve : & 'a Resolve , interface : Option < (InterfaceId , & 'a WorldKey) > , wasm_import_module : Option < & 'a str > , }
};
}
