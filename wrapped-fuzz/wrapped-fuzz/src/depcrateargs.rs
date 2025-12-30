// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Clone , Parser , Debug)] struct Args { # [doc = " Disable comparison with C++ (LLVM's original) APFloat"] # [arg (long)] ignore_cxx : bool , # [doc = " Disable comparison with hardware floating-point"] # [arg (long)] ignore_hard : bool , # [doc = " Disable erasure of NaN sign mismatches with hardware floating-point operations"] # [arg (long)] strict_hard_nan_sign : bool , # [doc = " Disable erasure of \"which NaN input propagates\" mismatches with hardware floating-point operations"] # [arg (long)] strict_hard_nan_input_choice : bool , # [doc = " Hide FMA NaN mismatches for `a * b + NaN` when `a * b` generates a new NaN"] # [arg (long)] ignore_fma_nan_generate_vs_propagate : bool , # [command (subcommand)] command : Option < Commands > , }
};
}
