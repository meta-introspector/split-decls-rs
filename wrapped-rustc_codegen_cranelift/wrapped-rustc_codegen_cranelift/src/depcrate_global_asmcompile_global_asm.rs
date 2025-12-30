// Generated macro for compile_global_asm (function)
macro_rules! Depcrate_global_asmcompile_global_asm {
() => {
// Module: crate::global_asm
// Provides: {"compile_global_asm"}
// Dependencies: {}
pub (crate) fn compile_global_asm (config : & GlobalAsmConfig , cgu_name : & str , global_asm : & str , invocation_temp : Option < & str > ,) -> Result < Option < PathBuf > , String > { if global_asm . is_empty () { return Ok (None) ; } let mut global_asm = global_asm . lines () . map (| line | if let Some (index) = line . find ("//") { & line [0 .. index] } else { line }) . collect :: < Vec < _ > > () . join ("\n") ; global_asm . push ('\n') ; let global_asm_object_file = add_file_stem_postfix (config . output_filenames . temp_path_for_cgu (OutputType :: Object , cgu_name , invocation_temp) , ".asm" ,) ; if option_env ! ("CG_CLIF_FORCE_GNU_AS") . is_some () { let mut child = Command :: new (& config . assembler) . arg ("-o") . arg (& global_asm_object_file) . stdin (Stdio :: piped ()) . spawn () . expect ("Failed to spawn `as`.") ; child . stdin . take () . unwrap () . write_all (global_asm . as_bytes ()) . unwrap () ; let status = child . wait () . expect ("Failed to wait for `as`.") ; if ! status . success () { return Err (format ! ("Failed to assemble `{}`" , global_asm)) ; } } else { let mut child = Command :: new (std :: env :: current_exe () . unwrap ()) . env_remove ("CARGO_MAKEFLAGS") . arg ("--target") . arg (& config . target) . arg ("--crate-type") . arg ("staticlib") . arg ("--emit") . arg ("obj") . arg ("-o") . arg (& global_asm_object_file) . arg ("-") . arg ("-Abad_asm_style") . arg ("-Zcodegen-backend=llvm") . stdin (Stdio :: piped ()) . spawn () . expect ("Failed to spawn `as`.") ; let mut stdin = child . stdin . take () . unwrap () ; stdin . write_all (br####"
                #![feature(decl_macro, no_core, rustc_attrs)]
                #![allow(internal_features)]
                #![no_core]
                #[rustc_builtin_macro]
                #[rustc_macro_transparency = "semitransparent"]
                macro global_asm() { /* compiler built-in */ }
                global_asm!(r###"
                "#### ,) . unwrap () ; stdin . write_all (global_asm . as_bytes ()) . unwrap () ; stdin . write_all (br####"
                "###);
                "#### ,) . unwrap () ; std :: mem :: drop (stdin) ; let status = child . wait () . expect ("Failed to wait for `as`.") ; if ! status . success () { return Err (format ! ("Failed to assemble `{}`" , global_asm)) ; } } Ok (Some (global_asm_object_file)) }
};
}
