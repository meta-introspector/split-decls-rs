// Generated macro for run_cargo_fmt (function)
macro_rules! Depcraterun_cargo_fmt {
() => {
// Module: crate
// Provides: {"run_cargo_fmt"}
// Dependencies: {}
pub fn run_cargo_fmt (packages : impl IntoIterator < Item = impl Display >) { let status = Command :: new ("cargo") . arg ("fmt") . args (packages . into_iter () . map (| package | format ! ("-p{package}"))) . current_dir (Path :: new (env ! ("CARGO_MANIFEST_DIR")) . parent () . unwrap ()) . status () . expect ("failed running cargo fmt") ; assert ! (status . success () , "failed running cargo fmt with exit code {status}") ; }
};
}
