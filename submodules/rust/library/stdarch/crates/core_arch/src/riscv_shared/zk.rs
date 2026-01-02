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
mkitem!{unsafe extern "unadjusted" { # [link_name = "llvm.riscv.sm4ed"] fn _sm4ed (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.sm4ks"] fn _sm4ks (rs1 : i32 , rs2 : i32 , bs : i32) -> i32 ; # [link_name = "llvm.riscv.sm3p0"] fn _sm3p0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sm3p1"] fn _sm3p1 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sig0"] fn _sha256sig0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sig1"] fn _sha256sig1 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sum0"] fn _sha256sum0 (rs1 : i32) -> i32 ; # [link_name = "llvm.riscv.sha256sum1"] fn _sha256sum1 (rs1 : i32) -> i32 ; }}
mkitem!{# [cfg (target_arch = "riscv32")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.xperm8.i32"] fn _xperm8_32 (rs1 : i32 , rs2 : i32) -> i32 ; # [link_name = "llvm.riscv.xperm4.i32"] fn _xperm4_32 (rs1 : i32 , rs2 : i32) -> i32 ; }}
mkitem!{# [cfg (target_arch = "riscv64")] unsafe extern "unadjusted" { # [link_name = "llvm.riscv.xperm8.i64"] fn _xperm8_64 (rs1 : i64 , rs2 : i64) -> i64 ; # [link_name = "llvm.riscv.xperm4.i64"] fn _xperm4_64 (rs1 : i64 , rs2 : i64) -> i64 ; }}

macro_rules! xperm8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xperm8 in module {}", module_path!());
    };
}

mkfn!{
    xperm8_introspect!();
    # [doc = " Byte-wise lookup of indicies into a vector in registers."] # [doc = ""] # [doc = " The xperm8 instruction operates on bytes. The rs1 register contains a vector of XLEN/8"] # [doc = " 8-bit elements. The rs2 register contains a vector of XLEN/8 8-bit indexes. The result is"] # [doc = " each element in rs2 replaced by the indexed element in rs1, or zero if the index into rs2"] # [doc = " is out of bounds."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.47"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbkx")] # [cfg_attr (test , assert_instr (xperm8))] # [inline] pub fn xperm8 (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _xperm8_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _xperm8_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! xperm4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xperm4 in module {}", module_path!());
    };
}

mkfn!{
    xperm4_introspect!();
    # [doc = " Nibble-wise lookup of indicies into a vector."] # [doc = ""] # [doc = " The xperm4 instruction operates on nibbles. The rs1 register contains a vector of XLEN/4"] # [doc = " 4-bit elements. The rs2 register contains a vector of XLEN/4 4-bit indexes. The result is"] # [doc = " each element in rs2 replaced by the indexed element in rs1, or zero if the index into rs2"] # [doc = " is out of bounds."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.48"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zbkx")] # [cfg_attr (test , assert_instr (xperm4))] # [inline] pub fn xperm4 (rs1 : usize , rs2 : usize) -> usize { # [cfg (target_arch = "riscv32")] unsafe { _xperm4_32 (rs1 as i32 , rs2 as i32) as usize } # [cfg (target_arch = "riscv64")] unsafe { _xperm4_64 (rs1 as i64 , rs2 as i64) as usize } }
}

macro_rules! sha256sig0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sig0 in module {}", module_path!());
    };
}

mkfn!{
    sha256sig0_introspect!();
    # [doc = " Implements the Sigma0 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.27"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sig0))] # [inline] pub fn sha256sig0 (rs1 : u32) -> u32 { unsafe { _sha256sig0 (rs1 as i32) as u32 } }
}

macro_rules! sha256sig1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sig1 in module {}", module_path!());
    };
}

mkfn!{
    sha256sig1_introspect!();
    # [doc = " Implements the Sigma1 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.28"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sig1))] # [inline] pub fn sha256sig1 (rs1 : u32) -> u32 { unsafe { _sha256sig1 (rs1 as i32) as u32 } }
}

macro_rules! sha256sum0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sum0 in module {}", module_path!());
    };
}

mkfn!{
    sha256sum0_introspect!();
    # [doc = " Implements the Sum0 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.29"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sum0))] # [inline] pub fn sha256sum0 (rs1 : u32) -> u32 { unsafe { _sha256sum0 (rs1 as i32) as u32 } }
}

macro_rules! sha256sum1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sha256sum1 in module {}", module_path!());
    };
}

mkfn!{
    sha256sum1_introspect!();
    # [doc = " Implements the Sum1 transformation function as used in the SHA2-256 hash function \\[49\\]"] # [doc = " (Section 4.1.2)."] # [doc = ""] # [doc = " This instruction is supported for both RV32 and RV64 base architectures. For RV32, the"] # [doc = " entire XLEN source register is operated on. For RV64, the low 32 bits of the source"] # [doc = " register are operated on, and the result sign extended to XLEN bits. Though named for"] # [doc = " SHA2-256, the instruction works for both the SHA2-224 and SHA2-256 parameterisations as"] # [doc = " described in \\[49\\]. This instruction must always be implemented such that its execution"] # [doc = " latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.30"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zknh")] # [cfg_attr (test , assert_instr (sha256sum1))] # [inline] pub fn sha256sum1 (rs1 : u32) -> u32 { unsafe { _sha256sum1 (rs1 as i32) as u32 } }
}

macro_rules! sm4ed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm4ed in module {}", module_path!());
    };
}

mkfn!{
    sm4ed_introspect!();
    # [doc = " Accelerates the block encrypt/decrypt operation of the SM4 block cipher \\[5, 31\\]."] # [doc = ""] # [doc = " Implements a T-tables in hardware style approach to accelerating the SM4 round function. A"] # [doc = " byte is extracted from rs2 based on bs, to which the SBox and linear layer transforms are"] # [doc = " applied, before the result is XOR’d with rs1 and written back to rd. This instruction"] # [doc = " exists on RV32 and RV64 base architectures. On RV64, the 32-bit result is sign extended to"] # [doc = " XLEN bits. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.43"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " Accelerates the round function `F` in the SM4 block cipher algorithm"] # [doc = ""] # [doc = " This instruction is included in extension `Zksed`. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " SM4ED(x, a, BS) = x ⊕ T(ai)"] # [doc = " ... where"] # [doc = " ai = a.bytes[BS]"] # [doc = " T(ai) = L(τ(ai))"] # [doc = " bi = τ(ai) = SM4-S-Box(ai)"] # [doc = " ci = L(bi) = bi ⊕ (bi ≪ 2) ⊕ (bi ≪ 10) ⊕ (bi ≪ 18) ⊕ (bi ≪ 24)"] # [doc = " SM4ED = (ci ≪ (BS * 8)) ⊕ x"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = " As is defined above, `T` is a combined transformation of non linear S-Box transform `τ`"] # [doc = " and linear layer transform `L`."] # [doc = ""] # [doc = " In the SM4 algorithm, the round function `F` is defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " F(x0, x1, x2, x3, rk) = x0 ⊕ T(x1 ⊕ x2 ⊕ x3 ⊕ rk)"] # [doc = " ... where"] # [doc = " T(A) = L(τ(A))"] # [doc = " B = τ(A) = (SM4-S-Box(a0), SM4-S-Box(a1), SM4-S-Box(a2), SM4-S-Box(a3))"] # [doc = " C = L(B) = B ⊕ (B ≪ 2) ⊕ (B ≪ 10) ⊕ (B ≪ 18) ⊕ (B ≪ 24)"] # [doc = " ```"] # [doc = ""] # [doc = " It can be implemented by `sm4ed` instruction like:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[cfg(any(target_arch = \"riscv32\", target_arch = \"riscv64\"))]"] # [doc = " # fn round_function(x0: u32, x1: u32, x2: u32, x3: u32, rk: u32) -> u32 {"] # [doc = " # #[cfg(target_arch = \"riscv32\")] use core::arch::riscv32::sm4ed;"] # [doc = " # #[cfg(target_arch = \"riscv64\")] use core::arch::riscv64::sm4ed;"] # [doc = " let a = x1 ^ x2 ^ x3 ^ rk;"] # [doc = " let c0 = sm4ed(x0, a, 0);"] # [doc = " let c1 = sm4ed(c0, a, 1); // c1 represents c[0..=1], etc."] # [doc = " let c2 = sm4ed(c1, a, 2);"] # [doc = " let c3 = sm4ed(c2, a, 3);"] # [doc = " return c3; // c3 represents c[0..=3]"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksed")] # [rustc_legacy_const_generics (2)] # [cfg_attr (test , assert_instr (sm4ed , BS = 0))] # [inline] pub fn sm4ed < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _sm4ed (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! sm4ks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm4ks in module {}", module_path!());
    };
}

mkfn!{
    sm4ks_introspect!();
    # [doc = " Accelerates the Key Schedule operation of the SM4 block cipher \\[5, 31\\] with `bs=0`."] # [doc = ""] # [doc = " Implements a T-tables in hardware style approach to accelerating the SM4 Key Schedule. A"] # [doc = " byte is extracted from rs2 based on bs, to which the SBox and linear layer transforms are"] # [doc = " applied, before the result is XOR’d with rs1 and written back to rd. This instruction"] # [doc = " exists on RV32 and RV64 base architectures. On RV64, the 32-bit result is sign extended to"] # [doc = " XLEN bits. This instruction must always be implemented such that its execution latency does"] # [doc = " not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.44"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " The `BS` parameter is expected to be a constant value and only the bottom 2 bits of `bs` are"] # [doc = " used."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " Accelerates the round function `F` in the SM4 block cipher algorithm"] # [doc = ""] # [doc = " This instruction is included in extension `Zksed`. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " SM4ED(x, a, BS) = x ⊕ T(ai)"] # [doc = " ... where"] # [doc = " ai = a.bytes[BS]"] # [doc = " T(ai) = L(τ(ai))"] # [doc = " bi = τ(ai) = SM4-S-Box(ai)"] # [doc = " ci = L(bi) = bi ⊕ (bi ≪ 2) ⊕ (bi ≪ 10) ⊕ (bi ≪ 18) ⊕ (bi ≪ 24)"] # [doc = " SM4ED = (ci ≪ (BS * 8)) ⊕ x"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = " As is defined above, `T` is a combined transformation of non linear S-Box transform `τ`"] # [doc = " and linear layer transform `L`."] # [doc = ""] # [doc = " In the SM4 algorithm, the round function `F` is defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " F(x0, x1, x2, x3, rk) = x0 ⊕ T(x1 ⊕ x2 ⊕ x3 ⊕ rk)"] # [doc = " ... where"] # [doc = " T(A) = L(τ(A))"] # [doc = " B = τ(A) = (SM4-S-Box(a0), SM4-S-Box(a1), SM4-S-Box(a2), SM4-S-Box(a3))"] # [doc = " C = L(B) = B ⊕ (B ≪ 2) ⊕ (B ≪ 10) ⊕ (B ≪ 18) ⊕ (B ≪ 24)"] # [doc = " ```"] # [doc = ""] # [doc = " It can be implemented by `sm4ed` instruction like:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[cfg(any(target_arch = \"riscv32\", target_arch = \"riscv64\"))]"] # [doc = " # fn round_function(x0: u32, x1: u32, x2: u32, x3: u32, rk: u32) -> u32 {"] # [doc = " # #[cfg(target_arch = \"riscv32\")] use core::arch::riscv32::sm4ed;"] # [doc = " # #[cfg(target_arch = \"riscv64\")] use core::arch::riscv64::sm4ed;"] # [doc = " let a = x1 ^ x2 ^ x3 ^ rk;"] # [doc = " let c0 = sm4ed(x0, a, 0);"] # [doc = " let c1 = sm4ed(c0, a, 1); // c1 represents c[0..=1], etc."] # [doc = " let c2 = sm4ed(c1, a, 2);"] # [doc = " let c3 = sm4ed(c2, a, 3);"] # [doc = " return c3; // c3 represents c[0..=3]"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksed")] # [rustc_legacy_const_generics (2)] # [cfg_attr (test , assert_instr (sm4ks , BS = 0))] # [inline] pub fn sm4ks < const BS : u8 > (rs1 : u32 , rs2 : u32) -> u32 { static_assert ! (BS < 4) ; unsafe { _sm4ks (rs1 as i32 , rs2 as i32 , BS as i32) as u32 } }
}

macro_rules! sm3p0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm3p0 in module {}", module_path!());
    };
}

mkfn!{
    sm3p0_introspect!();
    # [doc = " Implements the P0 transformation function as used in the SM3 hash function [4, 30]."] # [doc = ""] # [doc = " This instruction is supported for the RV32 and RV64 base architectures. It implements the"] # [doc = " P0 transform of the SM3 hash function [4, 30]. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.41"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " `P0` transformation function as is used in the SM3 hash algorithm"] # [doc = ""] # [doc = " This function is included in `Zksh` extension. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " P0(X) = X ⊕ (X ≪ 9) ⊕ (X ≪ 17)"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = ""] # [doc = " In the SM3 algorithm, the `P0` transformation is used as `E ← P0(TT2)` when the"] # [doc = " compression function `CF` uses the intermediate value `TT2` to calculate"] # [doc = " the variable `E` in one iteration for subsequent processes."] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksh")] # [cfg_attr (test , assert_instr (sm3p0))] # [inline] pub fn sm3p0 (rs1 : u32) -> u32 { unsafe { _sm3p0 (rs1 as i32) as u32 } }
}

macro_rules! sm3p1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sm3p1 in module {}", module_path!());
    };
}

mkfn!{
    sm3p1_introspect!();
    # [doc = " Implements the P1 transformation function as used in the SM3 hash function [4, 30]."] # [doc = ""] # [doc = " This instruction is supported for the RV32 and RV64 base architectures. It implements the"] # [doc = " P1 transform of the SM3 hash function [4, 30]. This instruction must always be implemented"] # [doc = " such that its execution latency does not depend on the data being operated on."] # [doc = ""] # [doc = " Source: RISC-V Cryptography Extensions Volume I: Scalar & Entropy Source Instructions"] # [doc = ""] # [doc = " Version: v1.0.1"] # [doc = ""] # [doc = " Section: 3.42"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " `P1` transformation function as is used in the SM3 hash algorithm"] # [doc = ""] # [doc = " This function is included in `Zksh` extension. It's defined as:"] # [doc = ""] # [doc = " ```text"] # [doc = " P1(X) = X ⊕ (X ≪ 15) ⊕ (X ≪ 23)"] # [doc = " ```"] # [doc = ""] # [doc = " where `⊕` represents 32-bit xor, and `≪ k` represents rotate left by `k` bits."] # [doc = ""] # [doc = " In the SM3 algorithm, the `P1` transformation is used to expand message,"] # [doc = " where expanded word `Wj` can be generated from the previous words."] # [doc = " The whole process can be described as the following pseudocode:"] # [doc = ""] # [doc = " ```text"] # [doc = " FOR j=16 TO 67"] # [doc = "     Wj ← P1(Wj−16 ⊕ Wj−9 ⊕ (Wj−3 ≪ 15)) ⊕ (Wj−13 ≪ 7) ⊕ Wj−6"] # [doc = " ENDFOR"] # [doc = " ```"] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] # [target_feature (enable = "zksh")] # [cfg_attr (test , assert_instr (sm3p1))] # [inline] pub fn sm3p1 (rs1 : u32) -> u32 { unsafe { _sm3p1 (rs1 as i32) as u32 } }
}