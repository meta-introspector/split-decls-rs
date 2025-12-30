// Generated macro for impl_227 (impl)
macro_rules! Depcrate_test_runnerimpl_227 {
() => {
// Module: crate::test_runner
// Provides: {"impl_227"}
// Dependencies: {}
impl CargoTestHandle { pub (crate) fn new (path : Option < & str > , options : CargoOptions , root : & AbsPath , test_target : TestTarget , sender : Sender < CargoTestMessage > ,) -> std :: io :: Result < Self > { let mut cmd = toolchain :: command (Tool :: Cargo . path () , root , & options . extra_env) ; cmd . env ("RUSTC_BOOTSTRAP" , "1") ; cmd . arg ("--color=always") ; cmd . arg ("test") ; cmd . arg ("--package") ; cmd . arg (& test_target . package) ; if let TargetKind :: Lib { .. } = test_target . kind { cmd . arg ("--lib") ; } else if let Some (cargo_target) = test_target . kind . as_cargo_target () { cmd . arg (format ! ("--{cargo_target}")) ; cmd . arg (& test_target . target) ; } else { tracing :: warn ! ("Running test for unknown cargo target {:?}" , test_target . kind) ; } cmd . arg ("--no-fail-fast") ; cmd . arg ("--manifest-path") ; cmd . arg (root . join ("Cargo.toml")) ; options . apply_on_command (& mut cmd) ; cmd . arg ("--") ; if let Some (path) = path { cmd . arg (path) ; } cmd . args (["-Z" , "unstable-options"]) ; cmd . arg ("--format=json") ; for extra_arg in options . extra_test_bin_args { cmd . arg (extra_arg) ; } Ok (Self { _handle : CommandHandle :: spawn (cmd , CargoTestOutputParser :: new (& test_target) , sender) ? , }) } }
};
}
