mkmod!{assert_instr, { 
                getname!(assert_instr);
                getsrc!(assert_instr);
                getpath!(assert_instr);
                get_deps!(assert_instr);
                get_crates!(assert_instr);
                mkinclude!(assert_instr);
                 
            }}
mkmod!{big_endian, { 
                getname!(big_endian);
                getsrc!(big_endian);
                getpath!(big_endian);
                get_deps!(big_endian);
                get_crates!(big_endian);
                mkinclude!(big_endian);
                 
            }}
mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkmod!{expression, { 
                getname!(expression);
                getsrc!(expression);
                getpath!(expression);
                get_deps!(expression);
                get_crates!(expression);
                mkinclude!(expression);
                 
            }}
mkmod!{fn_suffix, { 
                getname!(fn_suffix);
                getsrc!(fn_suffix);
                getpath!(fn_suffix);
                get_deps!(fn_suffix);
                get_crates!(fn_suffix);
                mkinclude!(fn_suffix);
                 
            }}
mkmod!{input, { 
                getname!(input);
                getsrc!(input);
                getpath!(input);
                get_deps!(input);
                get_crates!(input);
                mkinclude!(input);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{load_store_tests, { 
                getname!(load_store_tests);
                getsrc!(load_store_tests);
                getpath!(load_store_tests);
                get_deps!(load_store_tests);
                get_crates!(load_store_tests);
                mkinclude!(load_store_tests);
                 
            }}
mkmod!{matching, { 
                getname!(matching);
                getsrc!(matching);
                getpath!(matching);
                get_deps!(matching);
                get_crates!(matching);
                mkinclude!(matching);
                 
            }}
mkmod!{predicate_forms, { 
                getname!(predicate_forms);
                getsrc!(predicate_forms);
                getpath!(predicate_forms);
                get_deps!(predicate_forms);
                get_crates!(predicate_forms);
                mkinclude!(predicate_forms);
                 
            }}
mkmod!{typekinds, { 
                getname!(typekinds);
                getsrc!(typekinds);
                getpath!(typekinds);
                get_deps!(typekinds);
                get_crates!(typekinds);
                mkinclude!(typekinds);
                 
            }}
mkmod!{wildcards, { 
                getname!(wildcards);
                getsrc!(wildcards);
                getpath!(wildcards);
                get_deps!(wildcards);
                get_crates!(wildcards);
                mkinclude!(wildcards);
                 
            }}
mkmod!{wildstring, { 
                getname!(wildstring);
                getsrc!(wildstring);
                getpath!(wildstring);
                get_deps!(wildstring);
                get_crates!(wildstring);
                mkinclude!(wildstring);
                 
            }}
mkuse!{use intrinsic :: Test ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use quote :: quote ;}
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: { Command , Stdio } ;}
mkuse!{use walkdir :: WalkDir ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () -> Result < () , String > { parse_args () . into_iter () . map (| (filepath , out) | { File :: open (& filepath) . map (| f | (f , filepath , out)) . map_err (| e | format ! ("could not read input file: {e}")) }) . map (| res | { let (file , filepath , out) = res ? ; serde_yaml :: from_reader (file) . map (| input : input :: GeneratorInput | (input , filepath , out)) . map_err (| e | format ! ("could not parse input file: {e}")) }) . collect :: < Result < Vec < _ > , _ > > () ? . into_iter () . map (| (input , filepath , out) | { let intrinsics = input . intrinsics . into_iter () . map (| intrinsic | { intrinsic . generate_variants (& input . ctx) }) . try_collect () . map (| mut vv : Vec < _ > | { vv . sort_by_cached_key (| variants | { variants . first () . map_or_else (String :: default , | variant | { variant . signature . fn_name () . to_string () }) }) ; vv . into_iter () . flatten () . collect_vec () }) ? ; if filepath . ends_with ("sve.spec.yml") || filepath . ends_with ("sve2.spec.yml") { let loads = intrinsics . iter () . filter_map (| i | { if matches ! (i . test , Test :: Load (..)) { Some (i . clone ()) } else { None } }) . collect () ; let stores = intrinsics . iter () . filter_map (| i | { if matches ! (i . test , Test :: Store (..)) { Some (i . clone ()) } else { None } }) . collect () ; load_store_tests :: generate_load_store_tests (loads , stores , out . as_ref () . map (| o | make_tests_filepath (& filepath , o)) . as_ref ()) ? ; } Ok ((input :: GeneratorInput { intrinsics , ctx : input . ctx , } , filepath , out ,)) }) . try_for_each (| result : context :: Result < (input :: GeneratorInput , PathBuf , Option < PathBuf >) > | -> context :: Result { let (generated , filepath , out) = result ? ; let w = match out { Some (out) => Box :: new (File :: create (make_output_filepath (& filepath , & out)) . map_err (| e | format ! ("could not create output file: {e}")) ? ,) as Box < dyn Write > , None => Box :: new (std :: io :: stdout ()) as Box < dyn Write > , } ; generate_file (generated , w) . map_err (| e | format ! ("could not generate output file: {e}")) } ,) }
}

macro_rules! parse_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_args in module {}", module_path!());
    };
}

mkfn!{
    parse_args_introspect!();
    fn parse_args () -> Vec < (PathBuf , Option < PathBuf >) > { let mut args_it = std :: env :: args () . skip (1) ; assert ! (1 <= args_it . len () && args_it . len () <= 2 , "Usage: cargo run -p stdarch-gen-arm -- INPUT_DIR [OUTPUT_DIR]\n\
        where:\n\
        - INPUT_DIR contains a tree like: INPUT_DIR/<feature>/<arch>.spec.yml\n\
        - OUTPUT_DIR is a directory like: crates/core_arch/src/") ; let in_path = Path :: new (args_it . next () . unwrap () . as_str ()) . to_path_buf () ; assert ! (in_path . exists () && in_path . is_dir () , "invalid path {in_path:#?} given") ; let out_dir = if let Some (dir) = args_it . next () { let out_path = Path :: new (dir . as_str ()) . to_path_buf () ; assert ! (out_path . exists () && out_path . is_dir () , "invalid path {out_path:#?} given") ; Some (out_path) } else { std :: env :: current_exe () . map (| mut f | { f . pop () ; f . push ("../../crates/core_arch/src/") ; f . exists () . then_some (f) }) . ok () . flatten () } ; WalkDir :: new (in_path) . into_iter () . filter_map (Result :: ok) . filter (| f | f . file_type () . is_file ()) . map (| f | (f . into_path () , out_dir . clone ())) . collect () }
}

macro_rules! generate_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_file in module {}", module_path!());
    };
}

mkfn!{
    generate_file_introspect!();
    fn generate_file (generated_input : input :: GeneratorInput , mut out : Box < dyn Write > ,) -> std :: io :: Result < () > { write ! (out , r#"// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen-arm/spec/` and run the following command to re-generate this file:
//
// ```
// cargo run --bin=stdarch-gen-arm -- crates/stdarch-gen-arm/spec
// ```
#![allow(improper_ctypes)]

#[cfg(test)]
use stdarch_test::assert_instr;

use super::*;{uses_neon}

"# , uses_neon = if generated_input . ctx . uses_neon_types { "\nuse crate::core_arch::arch::aarch64::*;" } else { "" } ,) ? ; let intrinsics = generated_input . intrinsics ; format_code (out , quote ! { # (# intrinsics) * }) ? ; Ok (()) }
}

macro_rules! format_code_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_code in module {}", module_path!());
    };
}

mkfn!{
    format_code_introspect!();
    pub fn format_code (mut output : impl std :: io :: Write , input : impl std :: fmt :: Display ,) -> std :: io :: Result < () > { let proc = Command :: new ("rustfmt") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () ? ; write ! (proc . stdin . as_ref () . unwrap () , "{input}") ? ; output . write_all (proc . wait_with_output () ? . stdout . as_slice ()) }
}

macro_rules! make_output_filepath_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_output_filepath in module {}", module_path!());
    };
}

mkfn!{
    make_output_filepath_introspect!();
    # [doc = " Derive an output file path from an input file path and an output directory."] # [doc = ""] # [doc = " `in_filepath` is expected to have a structure like:"] # [doc = "     .../<feature>/<arch>.spec.yml"] # [doc = ""] # [doc = " The resulting output path will have a structure like:"] # [doc = "     <out_dirpath>/<arch>/<feature>/generated.rs"] # [doc = ""] # [doc = " Panics if the resulting name is empty, or if file_name() is not UTF-8."] fn make_output_filepath (in_filepath : & Path , out_dirpath : & Path) -> PathBuf { make_filepath (in_filepath , out_dirpath , | _name : & str | { "generated.rs" . to_owned () }) }
}

macro_rules! make_tests_filepath_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_tests_filepath in module {}", module_path!());
    };
}

mkfn!{
    make_tests_filepath_introspect!();
    fn make_tests_filepath (in_filepath : & Path , out_dirpath : & Path) -> PathBuf { make_filepath (in_filepath , out_dirpath , | name : & str | { format ! ("ld_st_tests_{name}.rs") }) }
}

macro_rules! make_filepath_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_filepath in module {}", module_path!());
    };
}

mkfn!{
    make_filepath_introspect!();
    fn make_filepath < F : FnOnce (& str) -> String > (in_filepath : & Path , out_dirpath : & Path , name_formatter : F ,) -> PathBuf { let mut parts = in_filepath . components () . rev () . map (| f | { f . as_os_str () . to_str () . expect ("Inputs must have valid, UTF-8 file_name()") }) ; let yml = parts . next () . expect ("Not enough input path elements.") ; let feature = parts . next () . expect ("Not enough input path elements.") ; let arch = yml . strip_suffix (".yml") . expect ("Expected .yml file input.") . strip_suffix (".spec") . expect ("Expected .spec.yml file input.") ; if arch . is_empty () { panic ! ("Extended ARCH.spec.yml file input.") ; } let mut output = out_dirpath . to_path_buf () ; output . push (arch) ; output . push (feature) ; output . push (name_formatter (arch)) ; output }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! infer_output_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_output_file in module {}", module_path!());
    };
}

mkfn!{
    infer_output_file_introspect!();
    # [test] fn infer_output_file () { macro_rules ! t { ($ src : expr , $ outdir : expr , $ dst : expr , $ ldst : expr) => { let src : PathBuf = $ src . iter () . collect () ; let outdir : PathBuf = $ outdir . iter () . collect () ; let dst : PathBuf = $ dst . iter () . collect () ; let ldst : PathBuf = $ ldst . iter () . collect () ; assert_eq ! (make_output_filepath (& src , & outdir) , dst) ; assert_eq ! (make_tests_filepath (& src , & outdir) , ldst) ; } ; } t ! (["FEAT" , "ARCH.spec.yml"] , [""] , ["ARCH" , "FEAT" , "generated.rs"] , ["ARCH" , "FEAT" , "ld_st_tests_ARCH.rs"]) ; t ! (["x" , "y" , "FEAT" , "ARCH.spec.yml"] , ["out"] , ["out" , "ARCH" , "FEAT" , "generated.rs"] , ["out" , "ARCH" , "FEAT" , "ld_st_tests_ARCH.rs"]) ; t ! (["p" , "q" , "FEAT" , "ARCH.spec.yml"] , ["a" , "b"] , ["a" , "b" , "ARCH" , "FEAT" , "generated.rs"] , ["a" , "b" , "ARCH" , "FEAT" , "ld_st_tests_ARCH.rs"]) ; t ! (["FEAT" , "ARCH.variant.spec.yml"] , ["out"] , ["out" , "ARCH.variant" , "FEAT" , "generated.rs"] , ["out" , "ARCH.variant" , "FEAT" , "ld_st_tests_ARCH.variant.rs"]) ; }
}

macro_rules! infer_output_file_no_stem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_output_file_no_stem in module {}", module_path!());
    };
}

mkfn!{
    infer_output_file_no_stem_introspect!();
    # [test] # [should_panic] fn infer_output_file_no_stem () { let src = PathBuf :: from ("FEAT/.spec.yml") ; make_output_filepath (& src , Path :: new ("")) ; }
}

macro_rules! infer_output_file_no_feat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_output_file_no_feat in module {}", module_path!());
    };
}

mkfn!{
    infer_output_file_no_feat_introspect!();
    # [test] # [should_panic] fn infer_output_file_no_feat () { let src = PathBuf :: from ("ARCH.spec.yml") ; make_output_filepath (& src , Path :: new ("")) ; }
}

macro_rules! infer_output_file_ldst_no_stem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_output_file_ldst_no_stem in module {}", module_path!());
    };
}

mkfn!{
    infer_output_file_ldst_no_stem_introspect!();
    # [test] # [should_panic] fn infer_output_file_ldst_no_stem () { let src = PathBuf :: from ("FEAT/.spec.yml") ; make_tests_filepath (& src , Path :: new ("")) ; }
}

macro_rules! infer_output_file_ldst_no_feat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_output_file_ldst_no_feat in module {}", module_path!());
    };
}

mkfn!{
    infer_output_file_ldst_no_feat_introspect!();
    # [test] # [should_panic] fn infer_output_file_ldst_no_feat () { let src = PathBuf :: from ("ARCH.spec.yml") ; make_tests_filepath (& src , Path :: new ("")) ; }
} 
            }}