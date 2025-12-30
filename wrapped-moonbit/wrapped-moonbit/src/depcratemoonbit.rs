// Generated macro for MoonBit (struct)
macro_rules! DepcrateMoonBit {
() => {
// Module: crate
// Provides: {"MoonBit"}
// Dependencies: {}
# [derive (Default)] pub struct MoonBit { opts : Opts , name : String , needs_cleanup : bool , import_interface_fragments : HashMap < String , Vec < InterfaceFragment > > , export_interface_fragments : HashMap < String , Vec < InterfaceFragment > > , import_world_fragments : Vec < InterfaceFragment > , export_world_fragments : Vec < InterfaceFragment > , sizes : SizeAlign , import_interface_names : HashMap < InterfaceId , String > , export_interface_names : HashMap < InterfaceId , String > , interface_ns : Ns , package_import : HashMap < String , Imports > , export : HashMap < String , String > , export_ns : Ns , return_area_size : ArchitectureSize , return_area_align : Alignment , futures : HashMap < String , HashSet < TypeId > > , is_async : bool , }
};
}
