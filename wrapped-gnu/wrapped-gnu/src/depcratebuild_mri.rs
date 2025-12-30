// Generated macro for build_mri (function)
macro_rules! Depcratebuild_mri {
() => {
// Module: crate
// Provides: {"build_mri"}
// Dependencies: {}
fn build_mri (output : & std :: path :: Path , ar : & str , libraries : & BTreeMap < String , BTreeMap < String , CallingConvention > > ,) { let mri_path = output . join ("unified.mri") ; let mut mri = std :: fs :: File :: create (& mri_path) . unwrap () ; println ! ("Generating {}" , mri_path . to_string_lossy ()) ; mri . write_all ("CREATE libwindows.0.53.0.a\n" . as_bytes ()) . unwrap () ; for library in libraries . keys () { mri . write_all (format ! ("ADDLIB lib{library}.a\n") . as_bytes ()) . unwrap () ; } mri . write_all (b"SAVE\nEND\n") . unwrap () ; let mut cmd = Command :: new (ar) ; cmd . current_dir (output) ; cmd . arg ("-M") ; cmd . stdin (std :: fs :: File :: open (& mri_path) . unwrap ()) ; cmd . output () . unwrap () ; std :: fs :: remove_file (& mri_path) . unwrap () ; }
};
}
