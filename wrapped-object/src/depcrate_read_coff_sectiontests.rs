// Generated macro for tests (module)
macro_rules! Depcrate_read_coff_sectiontests {
() => {
// Module: crate::read::coff::section
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn name_offset () { let mut section = pe :: ImageSectionHeader :: default () ; section . name = * b"xxxxxxxx" ; assert_eq ! (section . name_offset () , Ok (None)) ; section . name = * b"/0\0\0\0\0\0\0" ; assert_eq ! (section . name_offset () , Ok (Some (0))) ; section . name = * b"/9999999" ; assert_eq ! (section . name_offset () , Ok (Some (999_9999))) ; section . name = * b"//AAAAAA" ; assert_eq ! (section . name_offset () , Ok (Some (0))) ; section . name = * b"//D/////" ; assert_eq ! (section . name_offset () , Ok (Some (0xffff_ffff))) ; section . name = * b"//EAAAAA" ; assert ! (section . name_offset () . is_err ()) ; section . name = * b"////////" ; assert ! (section . name_offset () . is_err ()) ; } }
};
}
