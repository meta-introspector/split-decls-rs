// Generated macro for impl_90 (impl)
macro_rules! Depcrate_tarballerimpl_90 {
() => {
// Module: crate::tarballer
// Provides: {"impl_90"}
// Dependencies: {}
impl Tarballer { # [doc = " Generates the actual tarballs"] pub fn run (self) -> Result < () > { if let CompressionProfile :: NoOp = self . compression_profile { return Ok (()) ; } let tarball_name = self . output . clone () + ".tar" ; let encoder = CombinedEncoder :: new (self . compression_formats . iter () . map (| f | f . encode (& tarball_name , self . compression_profile)) . collect :: < Result < Vec < _ > > > () ? ,) ; let (mut dirs , mut files) = get_recursive_paths (& self . work_dir , & self . input) . context ("failed to collect file paths") ? ; dirs . sort () ; files . sort_by (| a , b | a . bytes () . rev () . cmp (b . bytes () . rev ())) ; let buf = BufWriter :: with_capacity (1024 * 1024 , encoder) ; let mut builder = Builder :: new (buf) ; builder . mode (HeaderMode :: Deterministic) ; let pool = rayon :: ThreadPoolBuilder :: new () . num_threads (2) . build () . unwrap () ; pool . install (move | | { for path in dirs { let src = Path :: new (& self . work_dir) . join (& path) ; builder . append_dir (& path , & src) . with_context (| | format ! ("failed to tar dir '{}'" , src . display ())) ? ; } for path in files { let src = Path :: new (& self . work_dir) . join (& path) ; append_path (& mut builder , & src , & path , self . override_file_mtime) . with_context (| | format ! ("failed to tar file '{}'" , src . display ())) ? ; } builder . into_inner () . context ("failed to finish writing .tar stream") ? . into_inner () . ok () . unwrap () . finish () ? ; Ok (()) }) } }
};
}
