// Generated macro for interface_name (function)
macro_rules! Depcrate_world_generatorinterface_name {
() => {
// Module: crate::world_generator
// Provides: {"interface_name"}
// Dependencies: {}
fn interface_name (csharp : & mut CSharp , resolve : & Resolve , name : & WorldKey , direction : Direction ,) -> String { let pkg = match name { WorldKey :: Name (_) => None , WorldKey :: Interface (id) => { let pkg = resolve . interfaces [* id] . package . unwrap () ; Some (resolve . packages [pkg] . name . clone ()) } } ; let name = match name { WorldKey :: Name (name) => name . to_upper_camel_case () , WorldKey :: Interface (id) => resolve . interfaces [* id] . name . as_ref () . unwrap () . to_upper_camel_case () , } ; let namespace = match & pkg { Some (name) => { let mut ns = format ! ("{}.{}." , name . namespace . to_csharp_ident () , name . name . to_csharp_ident ()) ; if let Some (version) = & name . version { let v = version . to_string () . replace ('.' , "_") . replace ('-' , "_") . replace ('+' , "_") ; ns = format ! ("{}v{}." , ns , & v) ; } ns } None => String :: new () , } ; let world_namespace = & csharp . qualifier () ; format ! ("{}wit.{}.{}I{name}" , world_namespace , match direction { Direction :: Import => "imports" , Direction :: Export => "exports" , } , namespace) }
};
}
