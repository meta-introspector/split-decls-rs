mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.riscv.aes32esi"] fn _aes32esi (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.aes32esmi"] fn _aes32esmi (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.aes32dsi"] fn _aes32dsi (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.aes32dsmi"] fn _aes32dsmi (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.zip.i32"] fn _zip (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.unzip.i32"] fn _unzip (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sig0h"] fn _sha512sig0h (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sig0l"] fn _sha512sig0l (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sig1h"] fn _sha512sig1h (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sig1l"] fn _sha512sig1l (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sum0r"] fn _sha512sum0r (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.sha512sum1r"] fn _sha512sum1r (rs1 : i32 , rs2 : i32) -> i32 ; }}

macro_rules! aes32esi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes32esi in module {}", module_path!());
    };
}

mkfn!{
    aes32esi_introspect!();
    # [doc = " AES final round encryption instruction for RV32."] # [doc = ""] # [doc = " This instruction sources a single byte from rs2 according to bs. To this it applies the"] # [doc = " forward AES SBox operation, before XOR’ing the result with rs1. This instruction must"] # [doc = " always be implemented such that its execution latency does not depend on the data being"] # [doc = " operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.3"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [target_feature (enable = "zkne")] # [rustc_legacy_const_generics (2)] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes32esi < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _aes32esi (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! aes32esmi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes32esmi in module {}", module_path!());
    };
}

mkfn!{
    aes32esmi_introspect!();
    # [doc = " AES middle round encryption instruction for RV32 with."] # [doc = ""] # [doc = " This instruction sources a single byte from rs2 according to bs. To this it applies the"] # [doc = " forward AES SBox operation, and a partial forward MixColumn, before XOR’ing the result with"] # [doc = " rs1. This instruction must always be implemented such that its execution latency does not"] # [doc = " depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.4"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `bs` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [target_feature (enable = "zkne")] # [rustc_legacy_const_generics (2)] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes32esmi < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _aes32esmi (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! aes32dsi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes32dsi in module {}", module_path!());
    };
}

mkfn!{
    aes32dsi_introspect!();
    # [doc = " AES final round decryption instruction for RV32."] # [doc = ""] # [doc = " This instruction sources a single byte from rs2 according to bs. To this it applies the"] # [doc = " inverse AES SBox operation, and XOR’s the result with rs1. This instruction must always be"] # [doc = " implemented such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.1"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [target_feature (enable = "zknd")] # [rustc_legacy_const_generics (2)] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes32dsi < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _aes32dsi (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! aes32dsmi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aes32dsmi in module {}", module_path!());
    };
}

mkfn!{
    aes32dsmi_introspect!();
    # [doc = " AES middle round decryption instruction for RV32."] # [doc = ""] # [doc = " This instruction sources a single byte from rs2 according to bs. To this it applies the"] # [doc = " inverse AES SBox operation, and a partial inverse MixColumn, before XOR’ing the result with"] # [doc = " rs1. This instruction must always be implemented such that its execution latency does not"] # [doc = " depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.2"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [target_feature (enable = "zknd")] # [rustc_legacy_const_generics (2)] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn aes32dsmi < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _aes32dsmi (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! zip_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zip in module {}", module_path!());
    };
}

mkfn!{
    zip_introspect!();
    # [doc = " Place upper/lower halves of the source register into odd/even bits of the destination"] # [doc = " respectivley."] # [doc = ""] # [doc = " This instruction places bits in the low half of the source register into the even bit"] # [doc = " positions of the destination, and bits in the high half of the source register into the odd"] # [doc = " bit positions of the destination. It is the inverse of the unzip instruction. This"] # [doc = " instruction is available only on RV32."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.49"] # [target_feature (enable = "zbkb")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zip (rs : u32) -> u32 { unsafe { _zip (rs as i32) as u32 } }
}

macro_rules! unzip_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unzip in module {}", module_path!());
    };
}

mkfn!{
    unzip_introspect!();
    # [doc = " Place odd and even bits of the source word into upper/lower halves of the destination."] # [doc = ""] # [doc = " This instruction places the even bits of the source register into the low half of the"] # [doc = " destination, and the odd bits of the source into the high bits of the destination. It is"] # [doc = " the inverse of the zip instruction. This instruction is available only on RV32."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.45"] # [target_feature (enable = "zbkb")] # [cfg_attr (test , assert_instr (unzip))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn unzip (rs : u32) -> u32 { unsafe { _unzip (rs as i32) as u32 } }
}

macro_rules! sha512sig0h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig0h in module {}", module_path!());
    };
}

mkfn!{
    sha512sig0h_introspect!();
    # [doc = " Implements the high half of the Sigma0 transformation, as used in the SHA2-512 hash"] # [doc = " function \\[49\\] (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sigma0 transform of the"] # [doc = " SHA2-512 hash function in conjunction with the sha512sig0l instruction. The transform is a"] # [doc = " 64-bit to 64-bit function, so the input and output are each represented by two 32-bit"] # [doc = " registers. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.31"] # [target_feature (enable = "zknh")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig0h (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sig0h (rs1 as i32 , rs2 as i32) as u32 } }
}

macro_rules! sha512sig0l_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig0l in module {}", module_path!());
    };
}

mkfn!{
    sha512sig0l_introspect!();
    # [doc = " Implements the low half of the Sigma0 transformation, as used in the SHA2-512 hash function"] # [doc = " \\[49\\] (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sigma0 transform of the"] # [doc = " SHA2-512 hash function in conjunction with the sha512sig0h instruction. The transform is a"] # [doc = " 64-bit to 64-bit function, so the input and output are each represented by two 32-bit"] # [doc = " registers. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.32"] # [target_feature (enable = "zknh")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig0l (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sig0l (rs1 as i32 , rs2 as i32) as u32 } }
}

macro_rules! sha512sig1h_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig1h in module {}", module_path!());
    };
}

mkfn!{
    sha512sig1h_introspect!();
    # [doc = " Implements the high half of the Sigma1 transformation, as used in the SHA2-512 hash"] # [doc = " function \\[49\\] (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sigma1 transform of the"] # [doc = " SHA2-512 hash function in conjunction with the sha512sig1l instruction. The transform is a"] # [doc = " 64-bit to 64-bit function, so the input and output are each represented by two 32-bit"] # [doc = " registers. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.33"] # [target_feature (enable = "zknh")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig1h (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sig1h (rs1 as i32 , rs2 as i32) as u32 } }
}

macro_rules! sha512sig1l_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sig1l in module {}", module_path!());
    };
}

mkfn!{
    sha512sig1l_introspect!();
    # [doc = " Implements the low half of the Sigma1 transformation, as used in the SHA2-512 hash function"] # [doc = " \\[49\\] (Section 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sigma1 transform of the"] # [doc = " SHA2-512 hash function in conjunction with the sha512sig1h instruction. The transform is a"] # [doc = " 64-bit to 64-bit function, so the input and output are each represented by two 32-bit"] # [doc = " registers. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.34"] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha512sig1l))] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sig1l (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sig1l (rs1 as i32 , rs2 as i32) as u32 } }
}

macro_rules! sha512sum0r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sum0r in module {}", module_path!());
    };
}

mkfn!{
    sha512sum0r_introspect!();
    # [doc = " Implements the Sum0 transformation, as used in the SHA2-512 hash function \\[49\\] (Section"] # [doc = " 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sum0 transform of the"] # [doc = " SHA2-512 hash function. The transform is a 64-bit to 64-bit function, so the input and"] # [doc = " output is represented by two 32-bit registers. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.35"] # [target_feature (enable = "zknh")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sum0r (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sum0r (rs1 as i32 , rs2 as i32) as u32 } }
}

macro_rules! sha512sum1r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha512sum1r in module {}", module_path!());
    };
}

mkfn!{
    sha512sum1r_introspect!();
    # [doc = " Implements the Sum1 transformation, as used in the SHA2-512 hash function \\[49\\] (Section"] # [doc = " 4.1.3)."] # [doc = ""] # [doc = " This instruction is implemented on RV32 only. Used to compute the Sum1 transform of the"] # [doc = " SHA2-512 hash function. The transform is a 64-bit to 64-bit function, so the input and"] # [doc = " output is represented by two 32-bit registers. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.36"] # [target_feature (enable = "zknh")] # [inline] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sha512sum1r (rs1 : u32 , rs2 : u32) -> u32 { unsafe { _sha512sum1r (rs1 as i32 , rs2 as i32) as u32 } }
}