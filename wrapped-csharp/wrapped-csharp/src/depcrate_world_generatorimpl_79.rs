// Generated macro for impl_79 (impl)
macro_rules! Depcrate_world_generatorimpl_79 {
() => {
// Module: crate::world_generator
// Provides: {"impl_79"}
// Dependencies: {}
impl CSharp { pub (crate) fn access_modifier (& self) -> & 'static str { if self . opts . internal { "internal" } else { "public" } } pub (crate) fn qualifier (& self) -> String { let world = self . name . to_upper_camel_case () ; format ! ("{world}World.") } fn interface < 'a > (& 'a mut self , resolve : & 'a Resolve , name : & 'a str , direction : Direction ,) -> InterfaceGenerator < 'a > { InterfaceGenerator { src : String :: new () , csharp_interop_src : String :: new () , stub : String :: new () , csharp_gen : self , resolve , name , direction , } } pub (crate) fn get_class_name_from_qualified_name (qualified_type : & str) -> (String , String) { let parts : Vec < & str > = qualified_type . split ('.') . collect () ; if let Some (last_part) = parts . last () { let mut qualifier = qualified_type . strip_suffix (last_part) ; if qualifier . is_some () { qualifier = qualifier . unwrap () . strip_suffix (".") ; } (qualifier . unwrap_or ("") . to_string () , last_part . to_string ()) } else { (String :: new () , String :: new ()) } } }
};
}
