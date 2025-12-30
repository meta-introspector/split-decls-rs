// Generated macro for Descriptor (struct)
macro_rules! Depcrate_codegenDescriptor {
() => {
// Module: crate::codegen
// Provides: {"Descriptor"}
// Dependencies: {}
# [doc = " Emits the necessary glue tokens for \"descriptor\", generating an appropriate"] # [doc = " symbol name as well as attributes around the descriptor function itself."] struct Descriptor < 'a , T > { ident : & 'a Ident , inner : T , attrs : Vec < syn :: Attribute > , wasm_bindgen : & 'a syn :: Path , }
};
}
