// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl CommandExt for Command { fn output_or_exit (& mut self) -> Output { if let Ok (output) = self . output () { if ! output . status . success () { eprintln ! ("[cargo-zerocopy] failed while capturing output from command: {:?}" , self) ; let stdout = std :: str :: from_utf8 (& output . stdout) . unwrap () ; let stderr = std :: str :: from_utf8 (& output . stderr) . unwrap () ; eprintln ! ("[cargo-zerocopy] stdout: {stdout}") ; eprintln ! ("[cargo-zerocopy] stderr: {stderr}") ; process :: exit (output . status . code () . unwrap_or (1)) ; } output } else { eprintln ! ("[cargo-zerocopy] failed to run command: {:?}" , self) ; process :: exit (1) ; } } fn execute (& mut self) { if let Ok (status) = self . status () { if ! status . success () { eprintln ! ("[cargo-zerocopy] failed while executing command: {:?}" , self) ; process :: exit (status . code () . unwrap_or (1)) ; } } else { eprintln ! ("[cargo-zerocopy] failed to run command: {:?}" , self) ; process :: exit (1) ; } } }
};
}
