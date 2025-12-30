// Generated macro for FunctionBindgen (struct)
macro_rules! DepcrateFunctionBindgen {
() => {
// Module: crate
// Provides: {"FunctionBindgen"}
// Dependencies: {}
struct FunctionBindgen < 'a , 'b > { gen : & 'b mut CppInterfaceGenerator < 'a > , params : Vec < String > , tmp : usize , namespace : Vec < String > , src : Source , block_storage : Vec < wit_bindgen_core :: Source > , # [doc = " intermediate calculations for contained objects"] blocks : Vec < (String , Vec < String >) > , payloads : Vec < String > , variant : AbiVariant , cabi_post : Option < CabiPostInformation > , needs_dealloc : bool , leak_on_insertion : Option < String > , }
};
}
