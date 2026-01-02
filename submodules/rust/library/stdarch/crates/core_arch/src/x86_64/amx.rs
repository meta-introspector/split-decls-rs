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

macro_rules! _tile_loadconfig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_loadconfig in module {}", module_path!());
    };
}

mkfn!{
    _tile_loadconfig_introspect!();
    # [doc = " Load tile configuration from a 64-byte memory location specified by mem_addr."] # [doc = " The tile configuration format is specified below, and includes the tile type pallette,"] # [doc = " the number of bytes per row, and the number of rows. If the specified pallette_id is zero,"] # [doc = " that signifies the init state for both the tile config and the tile data, and the tiles are zeroed."] # [doc = " Any invalid configurations will result in #GP fault."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_loadconfig&ig_expand=6875)"] # [inline] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (ldtilecfg))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_loadconfig (mem_addr : * const u8) { ldtilecfg (mem_addr) ; }
}

macro_rules! _tile_storeconfig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_storeconfig in module {}", module_path!());
    };
}

mkfn!{
    _tile_storeconfig_introspect!();
    # [doc = " Stores the current tile configuration to a 64-byte memory location specified by mem_addr."] # [doc = " The tile configuration format is specified below, and includes the tile type pallette,"] # [doc = " the number of bytes per row, and the number of rows. If tiles are not configured, all zeroes will be stored to memory."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_storeconfig&ig_expand=6879)"] # [inline] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (sttilecfg))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_storeconfig (mem_addr : * mut u8) { sttilecfg (mem_addr) ; }
}

macro_rules! _tile_loadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_loadd in module {}", module_path!());
    };
}

mkfn!{
    _tile_loadd_introspect!();
    # [doc = " Load tile rows from memory specifieid by base address and stride into destination tile dst using the tile configuration previously configured via _tile_loadconfig."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_loadd&ig_expand=6877)"] # [inline] # [rustc_legacy_const_generics (0)] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (tileloadd , DST = 0))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_loadd < const DST : i32 > (base : * const u8 , stride : usize) { static_assert_uimm_bits ! (DST , 3) ; tileloadd64 (DST as i8 , base , stride) ; }
}

macro_rules! _tile_release_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_release in module {}", module_path!());
    };
}

mkfn!{
    _tile_release_introspect!();
    # [doc = " Release the tile configuration to return to the init state, which releases all storage it currently holds."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_release&ig_expand=6878)"] # [inline] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (tilerelease))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_release () { tilerelease () ; }
}

macro_rules! _tile_stored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_stored in module {}", module_path!());
    };
}

mkfn!{
    _tile_stored_introspect!();
    # [doc = " Store the tile specified by src to memory specifieid by base address and stride using the tile configuration previously configured via _tile_loadconfig."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_stored&ig_expand=6881)"] # [inline] # [rustc_legacy_const_generics (0)] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (tilestored , DST = 0))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_stored < const DST : i32 > (base : * mut u8 , stride : usize) { static_assert_uimm_bits ! (DST , 3) ; tilestored64 (DST as i8 , base , stride) ; }
}

macro_rules! _tile_stream_loadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_stream_loadd in module {}", module_path!());
    };
}

mkfn!{
    _tile_stream_loadd_introspect!();
    # [doc = " Load tile rows from memory specifieid by base address and stride into destination tile dst using the tile configuration"] # [doc = " previously configured via _tile_loadconfig. This intrinsic provides a hint to the implementation that the data will"] # [doc = " likely not be reused in the near future and the data caching can be optimized accordingly."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_stream_loadd&ig_expand=6883)"] # [inline] # [rustc_legacy_const_generics (0)] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (tileloaddt1 , DST = 0))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_stream_loadd < const DST : i32 > (base : * const u8 , stride : usize) { static_assert_uimm_bits ! (DST , 3) ; tileloaddt164 (DST as i8 , base , stride) ; }
}

macro_rules! _tile_zero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_zero in module {}", module_path!());
    };
}

mkfn!{
    _tile_zero_introspect!();
    # [doc = " Zero the tile specified by tdest."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_zero&ig_expand=6885)"] # [inline] # [rustc_legacy_const_generics (0)] # [target_feature (enable = "amx-tile")] # [cfg_attr (test , assert_instr (tilezero , DST = 0))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_zero < const DST : i32 > () { static_assert_uimm_bits ! (DST , 3) ; tilezero (DST as i8) ; }
}

macro_rules! _tile_dpbf16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpbf16ps in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpbf16ps_introspect!();
    # [doc = " Compute dot-product of BF16 (16-bit) floating-point pairs in tiles a and b,"] # [doc = " accumulating the intermediate single-precision (32-bit) floating-point elements"] # [doc = " with elements in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbf16ps&ig_expand=6864)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-bf16")] # [cfg_attr (test , assert_instr (tdpbf16ps , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpbf16ps < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpbf16ps (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_dpbssd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpbssd in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpbssd_introspect!();
    # [doc = " Compute dot-product of bytes in tiles with a source/destination accumulator."] # [doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding"] # [doc = " signed 8-bit integers in b, producing 4 intermediate 32-bit results."] # [doc = " Sum these 4 results with the corresponding 32-bit integer in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbssd&ig_expand=6866)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-int8")] # [cfg_attr (test , assert_instr (tdpbssd , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpbssd < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpbssd (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_dpbsud_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpbsud in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpbsud_introspect!();
    # [doc = " Compute dot-product of bytes in tiles with a source/destination accumulator."] # [doc = " Multiply groups of 4 adjacent pairs of signed 8-bit integers in a with corresponding"] # [doc = " unsigned 8-bit integers in b, producing 4 intermediate 32-bit results."] # [doc = " Sum these 4 results with the corresponding 32-bit integer in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbsud&ig_expand=6868)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-int8")] # [cfg_attr (test , assert_instr (tdpbsud , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpbsud < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpbsud (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_dpbusd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpbusd in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpbusd_introspect!();
    # [doc = " Compute dot-product of bytes in tiles with a source/destination accumulator."] # [doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding"] # [doc = " signed 8-bit integers in b, producing 4 intermediate 32-bit results."] # [doc = " Sum these 4 results with the corresponding 32-bit integer in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbusd&ig_expand=6870)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-int8")] # [cfg_attr (test , assert_instr (tdpbusd , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpbusd < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpbusd (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_dpbuud_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpbuud in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpbuud_introspect!();
    # [doc = " Compute dot-product of bytes in tiles with a source/destination accumulator."] # [doc = " Multiply groups of 4 adjacent pairs of unsigned 8-bit integers in a with corresponding"] # [doc = " unsigned 8-bit integers in b, producing 4 intermediate 32-bit results."] # [doc = " Sum these 4 results with the corresponding 32-bit integer in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpbuud&ig_expand=6872)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-int8")] # [cfg_attr (test , assert_instr (tdpbuud , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpbuud < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpbuud (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_dpfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_dpfp16ps in module {}", module_path!());
    };
}

mkfn!{
    _tile_dpfp16ps_introspect!();
    # [doc = " Compute dot-product of FP16 (16-bit) floating-point pairs in tiles a and b,"] # [doc = " accumulating the intermediate single-precision (32-bit) floating-point elements"] # [doc = "  with elements in dst, and store the 32-bit result back to tile dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_dpfp16ps&ig_expand=6874)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-fp16")] # [cfg_attr (test , assert_instr (tdpfp16ps , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_dpfp16ps < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tdpfp16ps (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_cmmimfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_cmmimfp16ps in module {}", module_path!());
    };
}

mkfn!{
    _tile_cmmimfp16ps_introspect!();
    # [doc = " Perform matrix multiplication of two tiles containing complex elements and accumulate the results into a packed single precision tile."] # [doc = " Each dword element in input tiles a and b is interpreted as a complex number with FP16 real part and FP16 imaginary part."] # [doc = " Calculates the imaginary part of the result. For each possible combination of (row of a, column of b),"] # [doc = " it performs a set of multiplication and accumulations on all corresponding complex numbers (one from a and one from b)."] # [doc = " The imaginary part of the a element is multiplied with the real part of the corresponding b element, and the real part of"] # [doc = " the a element is multiplied with the imaginary part of the corresponding b elements. The two accumulated results are added,"] # [doc = " and then accumulated into the corresponding row and column of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_cmmimfp16ps&ig_expand=6860)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-complex")] # [cfg_attr (test , assert_instr (tcmmimfp16ps , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_cmmimfp16ps < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tcmmimfp16ps (DST as i8 , A as i8 , B as i8) ; }
}

macro_rules! _tile_cmmrlfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _tile_cmmrlfp16ps in module {}", module_path!());
    };
}

mkfn!{
    _tile_cmmrlfp16ps_introspect!();
    # [doc = " Perform matrix multiplication of two tiles containing complex elements and accumulate the results into a packed single precision tile."] # [doc = " Each dword element in input tiles a and b is interpreted as a complex number with FP16 real part and FP16 imaginary part."] # [doc = " Calculates the real part of the result. For each possible combination of (row of a, column of b),"] # [doc = " it performs a set of multiplication and accumulations on all corresponding complex numbers (one from a and one from b)."] # [doc = " The real part of the a element is multiplied with the real part of the corresponding b element, and the negated imaginary part of"] # [doc = " the a element is multiplied with the imaginary part of the corresponding b elements."] # [doc = " The two accumulated results are added, and then accumulated into the corresponding row and column of dst."] # [doc = ""] # [doc = " [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_tile_cmmrlfp16ps&ig_expand=6862)"] # [inline] # [rustc_legacy_const_generics (0 , 1 , 2)] # [target_feature (enable = "amx-complex")] # [cfg_attr (test , assert_instr (tcmmrlfp16ps , DST = 0 , A = 1 , B = 2))] # [unstable (feature = "x86_amx_intrinsics" , issue = "126622")] pub unsafe fn _tile_cmmrlfp16ps < const DST : i32 , const A : i32 , const B : i32 > () { static_assert_uimm_bits ! (DST , 3) ; static_assert_uimm_bits ! (A , 3) ; static_assert_uimm_bits ! (B , 3) ; tcmmrlfp16ps (DST as i8 , A as i8 , B as i8) ; }
}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.x86.ldtilecfg"] fn ldtilecfg (mem_addr : * const u8) ; # [link_name = "llvm.x86.sttilecfg"] fn sttilecfg (mem_addr : * mut u8) ; # [link_name = "llvm.x86.tileloadd64"] fn tileloadd64 (dst : i8 , base : * const u8 , stride : usize) ; # [link_name = "llvm.x86.tileloaddt164"] fn tileloaddt164 (dst : i8 , base : * const u8 , stride : usize) ; # [link_name = "llvm.x86.tilerelease"] fn tilerelease () ; # [link_name = "llvm.x86.tilestored64"] fn tilestored64 (dst : i8 , base : * mut u8 , stride : usize) ; # [link_name = "llvm.x86.tilezero"] fn tilezero (dst : i8) ; # [link_name = "llvm.x86.tdpbf16ps"] fn tdpbf16ps (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tdpbuud"] fn tdpbuud (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tdpbusd"] fn tdpbusd (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tdpbsud"] fn tdpbsud (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tdpbssd"] fn tdpbssd (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tdpfp16ps"] fn tdpfp16ps (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tcmmimfp16ps"] fn tcmmimfp16ps (dst : i8 , a : i8 , b : i8) ; # [link_name = "llvm.x86.tcmmrlfp16ps"] fn tcmmrlfp16ps (dst : i8 , a : i8 , b : i8) ; }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use crate :: core_arch :: x86 :: _mm_cvtness_sbh ;}
mkuse!{use crate :: core_arch :: x86_64 :: * ;}
mkuse!{use core :: mem :: transmute ;}
mkuse!{use stdarch_test :: simd_test ;}
mkuse!{# [cfg (target_os = "linux")] use syscalls :: { Sysno , syscall } ;}
mkitem!{mkstruct!{# [allow (non_camel_case_types)] # [repr (packed)] # [derive (Copy , Clone , Default , Debug , PartialEq)] struct __tilecfg { # [doc = " 0 `or` 1"] palette : u8 , start_row : u8 , # [doc = " reserved, must be zero"] reserved_a0 : [u8 ; 14] , # [doc = " number of bytes of one row in each tile"] colsb : [u16 ; 8] , # [doc = " reserved, must be zero"] reserved_b0 : [u16 ; 8] , # [doc = " number of rows in each tile"] rows : [u8 ; 8] , # [doc = " reserved, must be zero"] reserved_c0 : [u8 ; 8] , }}}
mkitem!{mkimpl!{impl __tilecfg { fn new (palette : u8 , start_row : u8 , colsb : [u16 ; 8] , rows : [u8 ; 8]) -> Self { Self { palette , start_row , reserved_a0 : [0u8 ; 14] , colsb , reserved_b0 : [0u16 ; 8] , rows , reserved_c0 : [0u8 ; 8] , } } const fn as_ptr (& self) -> * const u8 { self as * const Self as * const u8 } fn as_mut_ptr (& mut self) -> * mut u8 { self as * mut Self as * mut u8 } }}}

macro_rules! _init_amx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _init_amx in module {}", module_path!());
    };
}

mkfn!{
    _init_amx_introspect!();
    # [cfg (not (target_os = "linux"))] # [target_feature (enable = "amx-tile")] fn _init_amx () { }
}

macro_rules! _init_amx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _init_amx in module {}", module_path!());
    };
}

mkfn!{
    _init_amx_introspect!();
    # [cfg (target_os = "linux")] # [target_feature (enable = "amx-tile")] # [inline] unsafe fn _init_amx () { let mut ret : usize ; let mut xfeatures : usize = 0 ; ret = syscall ! (Sysno :: arch_prctl , 0x1022 , & mut xfeatures as * mut usize) . expect ("arch_prctl ARCH_GET_XCOMP_PERM syscall failed") ; if ret != 0 { panic ! ("Failed to get XFEATURES") ; } else { match 0b11 & (xfeatures >> 17) { 0 => panic ! ("AMX is not available") , 1 => { ret = syscall ! (Sysno :: arch_prctl , 0x1023 , 18) . expect ("arch_prctl ARCH_REQ_XCOMP_PERM syscall failed") ; if ret != 0 { panic ! ("Failed to enable AMX") ; } } 3 => { } _ => unreachable ! () , } } }
}

macro_rules! test_tile_loadconfig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_loadconfig in module {}", module_path!());
    };
}

mkfn!{
    test_tile_loadconfig_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_loadconfig () { let config = __tilecfg :: default () ; _tile_loadconfig (config . as_ptr ()) ; _tile_release () ; }
}

macro_rules! test_tile_storeconfig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_storeconfig in module {}", module_path!());
    };
}

mkfn!{
    test_tile_storeconfig_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_storeconfig () { let config = __tilecfg :: new (1 , 0 , [32 ; 8] , [8 ; 8]) ; _tile_loadconfig (config . as_ptr ()) ; let mut _config = __tilecfg :: default () ; _tile_storeconfig (_config . as_mut_ptr ()) ; _tile_release () ; assert_eq ! (config , _config) ; }
}

macro_rules! test_tile_zero_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_zero in module {}", module_path!());
    };
}

mkfn!{
    test_tile_zero_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_zero () { _init_amx () ; let mut config = __tilecfg :: default () ; config . palette = 1 ; config . colsb [0] = 64 ; config . rows [0] = 16 ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; let mut out = [[1_i8 ; 64] ; 16] ; _tile_stored :: < 0 > (& mut out as * mut [i8 ; 64] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (out , [[0 ; 64] ; 16]) ; }
}

macro_rules! test_tile_stored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_stored in module {}", module_path!());
    };
}

mkfn!{
    test_tile_stored_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_stored () { _init_amx () ; let mut config = __tilecfg :: default () ; config . palette = 1 ; config . colsb [0] = 64 ; config . rows [0] = 16 ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; let mut out = [[1_i8 ; 64] ; 16] ; _tile_stored :: < 0 > (& mut out as * mut [i8 ; 64] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (out , [[0 ; 64] ; 16]) ; }
}

macro_rules! test_tile_loadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_loadd in module {}", module_path!());
    };
}

mkfn!{
    test_tile_loadd_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_loadd () { _init_amx () ; let mut config = __tilecfg :: default () ; config . palette = 1 ; config . colsb [0] = 64 ; config . rows [0] = 16 ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; let mat = [1_i8 ; 1024] ; _tile_loadd :: < 0 > (& mat as * const i8 as * const u8 , 64) ; let mut out = [[0_i8 ; 64] ; 16] ; _tile_stored :: < 0 > (& mut out as * mut [i8 ; 64] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (out , [[1 ; 64] ; 16]) ; }
}

macro_rules! test_tile_stream_loadd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_stream_loadd in module {}", module_path!());
    };
}

mkfn!{
    test_tile_stream_loadd_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_stream_loadd () { _init_amx () ; let mut config = __tilecfg :: default () ; config . palette = 1 ; config . colsb [0] = 64 ; config . rows [0] = 16 ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; let mat = [1_i8 ; 1024] ; _tile_stream_loadd :: < 0 > (& mat as * const i8 as * const u8 , 64) ; let mut out = [[0_i8 ; 64] ; 16] ; _tile_stored :: < 0 > (& mut out as * mut [i8 ; 64] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (out , [[1 ; 64] ; 16]) ; }
}

macro_rules! test_tile_release_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_release in module {}", module_path!());
    };
}

mkfn!{
    test_tile_release_introspect!();
    # [simd_test (enable = "amx-tile")] unsafe fn test_tile_release () { _tile_release () ; }
}

macro_rules! test_tile_dpbf16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpbf16ps in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpbf16ps_introspect!();
    # [simd_test (enable = "amx-bf16,avx512f")] unsafe fn test_tile_dpbf16ps () { _init_amx () ; let bf16_1 : u16 = _mm_cvtness_sbh (1.0) . to_bits () ; let bf16_2 : u16 = _mm_cvtness_sbh (2.0) . to_bits () ; let ones : [u8 ; 1024] = transmute ([bf16_1 ; 512]) ; let twos : [u8 ; 1024] = transmute ([bf16_2 ; 512]) ; let mut res = [[0f32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const u8 , 64) ; _tile_dpbf16ps :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [f32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[64f32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_dpbssd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpbssd in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpbssd_introspect!();
    # [simd_test (enable = "amx-int8")] unsafe fn test_tile_dpbssd () { _init_amx () ; let ones = [- 1_i8 ; 1024] ; let twos = [- 2_i8 ; 1024] ; let mut res = [[0_i32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const i8 as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const i8 as * const u8 , 64) ; _tile_dpbssd :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [i32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[128_i32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_dpbsud_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpbsud in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpbsud_introspect!();
    # [simd_test (enable = "amx-int8")] unsafe fn test_tile_dpbsud () { _init_amx () ; let ones = [- 1_i8 ; 1024] ; let twos = [2_u8 ; 1024] ; let mut res = [[0_i32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const i8 as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const u8 , 64) ; _tile_dpbsud :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [i32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[- 128_i32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_dpbusd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpbusd in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpbusd_introspect!();
    # [simd_test (enable = "amx-int8")] unsafe fn test_tile_dpbusd () { _init_amx () ; let ones = [1_u8 ; 1024] ; let twos = [- 2_i8 ; 1024] ; let mut res = [[0_i32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const i8 as * const u8 , 64) ; _tile_dpbusd :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [i32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[- 128_i32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_dpbuud_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpbuud in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpbuud_introspect!();
    # [simd_test (enable = "amx-int8")] unsafe fn test_tile_dpbuud () { _init_amx () ; let ones = [1_u8 ; 1024] ; let twos = [2_u8 ; 1024] ; let mut res = [[0_i32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const u8 , 64) ; _tile_dpbuud :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [i32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[128_i32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_dpfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_dpfp16ps in module {}", module_path!());
    };
}

mkfn!{
    test_tile_dpfp16ps_introspect!();
    # [simd_test (enable = "amx-fp16")] unsafe fn test_tile_dpfp16ps () { _init_amx () ; let ones = [1f16 ; 512] ; let twos = [2f16 ; 512] ; let mut res = [[0f32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const f16 as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const f16 as * const u8 , 64) ; _tile_dpfp16ps :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [f32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[64f32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_cmmimfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_cmmimfp16ps in module {}", module_path!());
    };
}

mkfn!{
    test_tile_cmmimfp16ps_introspect!();
    # [simd_test (enable = "amx-complex")] unsafe fn test_tile_cmmimfp16ps () { _init_amx () ; let ones = [1f16 ; 512] ; let twos = [2f16 ; 512] ; let mut res = [[0f32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const f16 as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const f16 as * const u8 , 64) ; _tile_cmmimfp16ps :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [f32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[64f32 ; 16] ; 16]) ; }
}

macro_rules! test_tile_cmmrlfp16ps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_tile_cmmrlfp16ps in module {}", module_path!());
    };
}

mkfn!{
    test_tile_cmmrlfp16ps_introspect!();
    # [simd_test (enable = "amx-complex")] unsafe fn test_tile_cmmrlfp16ps () { _init_amx () ; let ones = [1f16 ; 512] ; let twos = [2f16 ; 512] ; let mut res = [[0f32 ; 16] ; 16] ; let mut config = __tilecfg :: default () ; config . palette = 1 ; (0 ..= 2) . for_each (| i | { config . colsb [i] = 64 ; config . rows [i] = 16 ; }) ; _tile_loadconfig (config . as_ptr ()) ; _tile_zero :: < 0 > () ; _tile_loadd :: < 1 > (& ones as * const f16 as * const u8 , 64) ; _tile_loadd :: < 2 > (& twos as * const f16 as * const u8 , 64) ; _tile_cmmrlfp16ps :: < 0 , 1 , 2 > () ; _tile_stored :: < 0 > (& mut res as * mut [f32 ; 16] as * mut u8 , 64) ; _tile_release () ; assert_eq ! (res , [[0f32 ; 16] ; 16]) ; }
} 
            }}