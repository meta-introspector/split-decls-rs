// Generated macro for object (function)
macro_rules! Depcrate_component_type_objectobject {
() => {
// Module: crate::component_type_object
// Provides: {"object"}
// Dependencies: {}
pub fn object (resolve : & Resolve , world : WorldId , world_name : & str , encoding : StringEncoding , suffix : Option < & str > ,) -> Result < Vec < u8 > > { let mut module = Module :: new () ; let mut types = TypeSection :: new () ; types . ty () . function ([] , []) ; module . section (& types) ; let mut funcs = FunctionSection :: new () ; funcs . function (0) ; module . section (& funcs) ; let mut code = CodeSection :: new () ; let mut func = Function :: new ([]) ; func . instruction (& wasm_encoder :: Instruction :: End) ; code . function (& func) ; module . section (& code) ; let mut producers = wasm_metadata :: Producers :: empty () ; producers . add ("processed-by" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION") ,) ; let data = wit_component :: metadata :: encode (resolve , world , encoding , Some (& producers)) . unwrap () ; let section_name = format ! ("component-type:{world_name}{}" , suffix . unwrap_or ("")) ; module . section (& CustomSection { name : std :: borrow :: Cow :: Borrowed (& section_name) , data : std :: borrow :: Cow :: Borrowed (data . as_slice ()) , }) ; let mut linking = LinkingSection :: new () ; let mut symbols = SymbolTable :: new () ; symbols . function (0 , 0 , Some (& linking_symbol (world_name))) ; linking . symbol_table (& symbols) ; module . section (& linking) ; Ok (module . finish ()) }
};
}
