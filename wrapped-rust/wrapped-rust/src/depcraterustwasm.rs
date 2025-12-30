// Generated macro for RustWasm (struct)
macro_rules! DepcrateRustWasm {
() => {
// Module: crate
// Provides: {"RustWasm"}
// Dependencies: {}
# [derive (Default)] struct RustWasm { types : Types , src_preamble : Source , src : Source , opts : Opts , import_modules : Vec < (String , Vec < String >) > , export_modules : Vec < (String , Vec < String >) > , skip : HashSet < String > , interface_names : HashMap < InterfaceId , InterfaceName > , # [doc = " Each imported and exported interface is stored in this map. Value indicates if last use was import."] interface_last_seen_as_import : HashMap < InterfaceId , bool > , import_funcs_called : bool , with_name_counter : usize , generated_types : HashSet < String > , world : Option < WorldId > , rt_module : IndexSet < RuntimeItem > , export_macros : Vec < (String , String) > , # [doc = " Maps wit interface and type names to their Rust identifiers"] with : GenerationConfiguration , future_payloads : IndexMap < String , String > , stream_payloads : IndexMap < String , String > , }
};
}
