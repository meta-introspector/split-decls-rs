// Generated macro for LintExtractor (struct)
macro_rules! DepcrateLintExtractor {
() => {
// Module: crate
// Provides: {"LintExtractor"}
// Dependencies: {}
pub struct LintExtractor < 'a > { # [doc = " Path to the `src` directory, where it will scan for `.rs` files to"] # [doc = " find lint declarations."] pub src_path : & 'a Path , # [doc = " Path where to save the output."] pub out_path : & 'a Path , # [doc = " Path to the `rustc` executable."] pub rustc_path : & 'a Path , # [doc = " The target arch to build the docs for."] pub rustc_target : & 'a str , # [doc = " The target linker overriding `rustc`'s default"] pub rustc_linker : Option < & 'a str > , # [doc = " Stage of the compiler that builds the docs (the stage of `rustc_path`)."] pub build_rustc_stage : u32 , # [doc = " Verbose output."] pub verbose : bool , # [doc = " Validate the style and the code example."] pub validate : bool , }
};
}
