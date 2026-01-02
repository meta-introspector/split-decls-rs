mkuse!{# [cfg (feature = "master")] use gccjit :: Context ;}
mkuse!{use rustc_codegen_ssa :: target_features ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}

macro_rules! gcc_features_by_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gcc_features_by_flags in module {}", module_path!());
    };
}

mkfn!{
    gcc_features_by_flags_introspect!();
    fn gcc_features_by_flags (sess : & Session , features : & mut Vec < String >) { target_features :: retpoline_features_by_flags (sess , features) ; }
}

macro_rules! global_gcc_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function global_gcc_features in module {}", module_path!());
    };
}

mkfn!{
    global_gcc_features_introspect!();
    # [doc = " The list of GCC features computed from CLI flags (`-Ctarget-cpu`, `-Ctarget-feature`,"] # [doc = " `--target` and similar)."] pub (crate) fn global_gcc_features (sess : & Session , diagnostics : bool) -> Vec < String > { let mut features = vec ! [] ; features . extend (sess . target . features . split (',') . filter (| v | ! v . is_empty ()) . map (String :: from)) ; target_features :: flag_to_backend_features (sess , diagnostics , | feature | to_gcc_features (sess , feature) , | feature , enable | { features . extend (to_gcc_features (sess , feature) . iter () . flat_map (| feat | to_gcc_features (sess , feat) . into_iter ()) . map (| feature | { if ! enable { format ! ("-{}" , feature) } else { feature . to_string () } } ,) ,) ; } ,) ; gcc_features_by_flags (sess , & mut features) ; features }
}

macro_rules! to_gcc_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_gcc_features in module {}", module_path!());
    };
}

mkfn!{
    to_gcc_features_introspect!();
    pub fn to_gcc_features < 'a > (sess : & Session , s : & 'a str) -> SmallVec < [& 'a str ; 2] > { let arch = if sess . target . arch == "x86_64" { "x86" } else { & * sess . target . arch } ; match (arch , s) { ("x86" , "x87") => smallvec ! [] , ("x86" , "sse4.2") => smallvec ! ["sse4.2" , "crc32"] , ("x86" , "pclmulqdq") => smallvec ! ["pclmul"] , ("x86" , "rdrand") => smallvec ! ["rdrnd"] , ("x86" , "bmi1") => smallvec ! ["bmi"] , ("x86" , "cmpxchg16b") => smallvec ! ["cx16"] , ("x86" , "avx512vaes") => smallvec ! ["vaes"] , ("x86" , "avx512gfni") => smallvec ! ["gfni"] , ("x86" , "avx512vpclmulqdq") => smallvec ! ["vpclmulqdq"] , ("x86" , "avx512vbmi2") => smallvec ! ["avx512vbmi2" , "avx512bw"] , ("x86" , "avx512bitalg") => smallvec ! ["avx512bitalg" , "avx512bw"] , ("aarch64" , "rcpc2") => smallvec ! ["rcpc-immo"] , ("aarch64" , "dpb") => smallvec ! ["ccpp"] , ("aarch64" , "dpb2") => smallvec ! ["ccdp"] , ("aarch64" , "frintts") => smallvec ! ["fptoint"] , ("aarch64" , "fcma") => smallvec ! ["complxnum"] , ("aarch64" , "pmuv3") => smallvec ! ["perfmon"] , ("aarch64" , "paca") => smallvec ! ["pauth"] , ("aarch64" , "pacg") => smallvec ! ["pauth"] , ("aarch64" , "f32mm") => smallvec ! ["f32mm" , "neon"] , ("aarch64" , "f64mm") => smallvec ! ["f64mm" , "neon"] , ("aarch64" , "fhm") => smallvec ! ["fp16fml" , "neon"] , ("aarch64" , "fp16") => smallvec ! ["fullfp16" , "neon"] , ("aarch64" , "jsconv") => smallvec ! ["jsconv" , "neon"] , ("aarch64" , "sve") => smallvec ! ["sve" , "neon"] , ("aarch64" , "sve2") => smallvec ! ["sve2" , "neon"] , ("aarch64" , "sve2-aes") => smallvec ! ["sve2-aes" , "neon"] , ("aarch64" , "sve2-sm4") => smallvec ! ["sve2-sm4" , "neon"] , ("aarch64" , "sve2-sha3") => smallvec ! ["sve2-sha3" , "neon"] , ("aarch64" , "sve2-bitperm") => smallvec ! ["sve2-bitperm" , "neon"] , (_ , s) => smallvec ! [s] , } }
}

macro_rules! arch_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function arch_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    arch_to_gcc_introspect!();
    fn arch_to_gcc (name : & str) -> & str { match name { "M68000" => "68000" , "M68020" => "68020" , _ => name , } }
}

macro_rules! handle_native_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_native in module {}", module_path!());
    };
}

mkfn!{
    handle_native_introspect!();
    fn handle_native (name : & str) -> & str { if name != "native" { return arch_to_gcc (name) ; } # [cfg (feature = "master")] { let context = Context :: default () ; context . get_target_info () . arch () . unwrap () . to_str () . unwrap () } # [cfg (not (feature = "master"))] unimplemented ! () ; }
}

macro_rules! target_cpu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_cpu in module {}", module_path!());
    };
}

mkfn!{
    target_cpu_introspect!();
    pub fn target_cpu (sess : & Session) -> & str { match sess . opts . cg . target_cpu { Some (ref name) => handle_native (name) , None => handle_native (sess . target . cpu . as_ref ()) , } }
}