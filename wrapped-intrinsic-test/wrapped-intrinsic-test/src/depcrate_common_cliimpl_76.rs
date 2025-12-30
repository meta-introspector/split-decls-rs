// Generated macro for impl_76 (impl)
macro_rules! Depcrate_common_cliimpl_76 {
() => {
// Module: crate::common::cli
// Provides: {"impl_76"}
// Dependencies: {}
impl ProcessedCli { pub fn new (cli_options : Cli) -> Self { let filename = cli_options . input ; let runner = cli_options . runner . unwrap_or_default () ; let target = cli_options . target ; let linker = cli_options . linker ; let cxx_toolchain_dir = cli_options . cxx_toolchain_dir ; let sample_percentage = cli_options . sample_percentage ; let skip = if let Some (filename) = cli_options . skip { let data = std :: fs :: read_to_string (& filename) . expect ("Failed to open file") ; data . lines () . map (str :: trim) . filter (| s | ! s . contains ('#')) . map (String :: from) . collect_vec () } else { Default :: default () } ; let (toolchain , cpp_compiler) = if cli_options . generate_only { (None , None) } else { (Some (cli_options . toolchain . map_or_else (String :: new , | t | format ! ("+{t}")) ,) , Some (cli_options . cppcompiler) ,) } ; Self { toolchain , cpp_compiler , runner , target , linker , cxx_toolchain_dir , skip , filename , sample_percentage , } } }
};
}
