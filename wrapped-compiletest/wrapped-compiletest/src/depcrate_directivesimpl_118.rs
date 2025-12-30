// Generated macro for impl_118 (impl)
macro_rules! Depcrate_directivesimpl_118 {
() => {
// Module: crate::directives
// Provides: {"impl_118"}
// Dependencies: {}
impl EarlyProps { pub fn from_file (config : & Config , testfile : & Utf8Path) -> Self { let file = File :: open (testfile . as_std_path ()) . expect ("open test file to parse earlyprops") ; Self :: from_reader (config , testfile , file) } pub fn from_reader < R : Read > (config : & Config , testfile : & Utf8Path , rdr : R) -> Self { let mut props = EarlyProps :: default () ; let mut poisoned = false ; iter_directives (config . mode , & mut poisoned , testfile , rdr , & mut | DirectiveLine { line_number , raw_directive : ln , .. } | { parse_and_update_aux (config , ln , testfile , line_number , & mut props . aux) ; config . parse_and_update_revisions (testfile , line_number , ln , & mut props . revisions) ; } ,) ; if poisoned { eprintln ! ("errors encountered during EarlyProps parsing: {}" , testfile) ; panic ! ("errors encountered during EarlyProps parsing") ; } props } }
};
}
