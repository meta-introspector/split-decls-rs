// Generated macro for run_llvm_config (function)
macro_rules! Depcrate_supportrun_llvm_config {
() => {
// Module: crate::support
// Provides: {"run_llvm_config"}
// Dependencies: {}
# [doc = " Runs `llvm-config`, returning the `stdout` output if successful."] fn run_llvm_config (arguments : & [& str]) -> Result < String , String > { let config = env :: var ("LLVM_CONFIG_PATH") . unwrap_or_else (| _ | "llvm-config" . to_string ()) ; run (& config , arguments) . map (| (o , _) | o) }
};
}
