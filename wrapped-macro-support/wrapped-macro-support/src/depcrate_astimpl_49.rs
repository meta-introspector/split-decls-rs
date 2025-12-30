// Generated macro for impl_49 (impl)
macro_rules! Depcrate_astimpl_49 {
() => {
// Module: crate::ast
// Provides: {"impl_49"}
// Dependencies: {}
impl Function { # [doc = " If the rust object has a `fn xxx(&self) -> MyType` method, get the name for a getter in"] # [doc = " javascript (in this case `xxx`, so you can write `val = obj.xxx`)"] pub fn infer_getter_property (& self) -> & str { & self . name } # [doc = " If the rust object has a `fn set_xxx(&mut self, MyType)` style method, get the name"] # [doc = " for a setter in javascript (in this case `xxx`, so you can write `obj.xxx = val`)"] pub fn infer_setter_property (& self) -> Result < String , Diagnostic > { let name = self . name . to_string () ; if ! name . starts_with ("set_") { bail_span ! (syn :: token :: Pub (self . name_span) , "setters must start with `set_`, found: {}" , name ,) ; } Ok (name [4 ..] . to_string ()) } }
};
}
