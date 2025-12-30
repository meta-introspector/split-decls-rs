// Generated macro for get_xml_intrinsics (function)
macro_rules! Depcrate_x86_xml_parserget_xml_intrinsics {
() => {
// Module: crate::x86::xml_parser
// Provides: {"get_xml_intrinsics"}
// Dependencies: {}
pub fn get_xml_intrinsics (filename : & Path ,) -> Result < Vec < Intrinsic < X86IntrinsicType > > , Box < dyn std :: error :: Error > > { let file = std :: fs :: File :: open (filename) ? ; let reader = std :: io :: BufReader :: new (file) ; let data : Data = quick_xml :: de :: from_reader (reader) . expect ("failed to deserialize the source XML file") ; let parsed_intrinsics : Vec < Intrinsic < X86IntrinsicType > > = data . intrinsics . into_iter () . filter (| intrinsic | { intrinsic . tech != "SVML" && intrinsic . tech != "MMX" && ! intrinsic . cpuid . contains (& "MPX" . to_string ()) && intrinsic . return_data . type_data != "__m64" && ! intrinsic . parameters . iter () . any (| param | param . type_data . contains ("__m64")) }) . filter_map (| intr | { xml_to_intrinsic (intr) . ok () }) . collect () ; Ok (parsed_intrinsics) }
};
}
