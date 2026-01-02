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
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use super :: ResultCache ;}

macro_rules! save_reduction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function save_reduction in module {}", module_path!());
    };
}

mkfn!{
    save_reduction_introspect!();
    # [doc = " Saves a reduced file for a given `stage`"] fn save_reduction (lines : & [String] , path : & Path , stage : & str) { let mut path = path . to_path_buf () ; path . set_extension (format ! ("rs.{stage}")) ; let mut file = std :: fs :: File :: create (& path) . expect ("Could not create the reduced example file") ; for line in lines { file . write_all (line . as_bytes ()) . expect ("Could not save the reduced example") ; } }
}

macro_rules! test_reduction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_reduction in module {}", module_path!());
    };
}

mkfn!{
    test_reduction_introspect!();
    # [doc = " Checks if a given reduction is valid."] fn test_reduction (lines : & [String] , path : & Path , cache : & mut ResultCache) -> bool { let mut path = path . to_path_buf () ; path . set_extension ("rs_reduced") ; let mut file = std :: fs :: File :: create (& path) . expect ("Could not create the reduced example file") ; for line in lines { file . write_all (line . as_bytes ()) . expect ("Could not save the reduced example") ; } let res = super :: test_cached (& path , false , cache) ; let Ok (Err (_)) = res else { return false ; } ; true }
}

macro_rules! remove_dup_assign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dup_assign in module {}", module_path!());
    };
}

mkfn!{
    remove_dup_assign_introspect!();
    # [doc = " Removes duplicate assignments in bulk."] # [doc = " If a line A = B is followed directly by A = C,"] # [doc = " then removing the first line ought to be fully sound,"] # [doc = " and not change the behaviour of the program at all. Detect & remove such lines."] fn remove_dup_assign (file : & mut Vec < String > , path : & PathBuf , starts : usize , ends : usize , cache : & mut ResultCache ,) { let mut file_copy = file . clone () ; let mut reduction_count = 0 ; if ends - starts < 8 { return ; } for index in starts .. ends { let Some ((prefix , _)) = file_copy [index] . split_once ('=') else { continue ; } ; let Some ((prefix2 , postifx2)) = file_copy [index + 1] . split_once ('=') else { continue ; } ; let prefix = prefix . trim () ; let prefix2 = prefix2 . trim () ; if prefix == prefix2 && ! postifx2 . contains (prefix) { file_copy [index] = "" . into () ; reduction_count += 1 ; } } if reduction_count == 0 { return ; } if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by {reduction_count} lines `remove_dup_assign`") ; * file = file_copy ; } else { remove_dup_assign (file , path , starts , (starts + ends) / 2 , cache) ; remove_dup_assign (file , path , (starts + ends) / 2 , ends , cache) ; } save_reduction (file , path , "remove_dup_assign") ; }
}

macro_rules! remove_dump_var_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dump_var in module {}", module_path!());
    };
}

mkfn!{
    remove_dump_var_introspect!();
    # [doc = " Removes all the unneeded calls to `dump_var`. This is not something tools like `cvise` can do,"] # [doc = " but it greately speeds up MIR interpretation + native execution."] fn remove_dump_var (file : & mut Vec < String > , path : & PathBuf) { let mut curr = 0 ; while curr < file . len () { let Some (line) = file [curr ..] . iter () . position (| line | line . contains ("dump_var")) else { break ; } ; let line = line + curr ; let mut file_copy = file . clone () ; file_copy . remove (line) ; file_copy . remove (line) ; file_copy . remove (line) ; let mut uncached = None ; if test_reduction (& file_copy , path , & mut uncached) { println ! ("Reduced {path:?} by 3 lines `remove_dump_var`") ; * file = file_copy ; curr = line ; } else { curr = line + 1 ; } } save_reduction (file , path , "remove_dump_var") ; }
}

macro_rules! match_to_goto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function match_to_goto in module {}", module_path!());
    };
}

mkfn!{
    match_to_goto_introspect!();
    # [doc = " Replaces matches with gotos where possible."] # [doc = " This exploits some properties of rustlantis(match arm order),"] # [doc = " and is only soundly applicable to MIR generated by it."] # [doc = " Still, it is not something `cvise` can do, but it simplifies the code a ton."] fn match_to_goto (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (match_starts) = file [curr ..] . iter () . position (| line | line . contains ("match")) else { break ; } ; let match_starts = match_starts + curr ; let Some (match_ends) = file [match_starts ..] . iter () . position (| line | line . contains ('}')) else { break ; } ; let match_ends = match_ends + match_starts ; let match_body = & file [match_starts .. match_ends] ; let jumps_to = & match_body [match_body . len () - 2] . trim () ; let Some ((_ , bb_ident)) = jumps_to . split_once ("bb") else { break ; } ; let bb_ident = bb_ident . trim_matches (',') ; let mut file_copy = file . clone () ; for _ in match_starts .. (match_ends + 1) { file_copy . remove (match_starts) ; } file_copy . insert (match_starts , format ! ("Goto(bb{bb_ident})\n")) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by {} lines `match_to_goto`" , match_ends - match_starts) ; * file = file_copy ; curr = match_starts ; } else { curr = match_ends ; } } save_reduction (file , path , "match_to_goto") ; }
}

macro_rules! block_abort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function block_abort in module {}", module_path!());
    };
}

mkfn!{
    block_abort_introspect!();
    # [doc = " At this point, we can try \"killing\" blocks, by replacing their bodies with calls to `abort`."] # [doc = " This is always sound(the program aborts, so no UB can occur after the block),"] # [doc = " and allows us to safely remove *a lot* of unneeded blocks."] fn block_abort (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (block_starts) = file [curr ..] . iter () . position (| line | line . starts_with ("bb") && line . trim_end () . ends_with (" = {")) else { break ; } ; let block_starts = block_starts + curr ; let Some (block_ends) = file [(block_starts + 1) ..] . iter () . position (| line | line . starts_with ("bb") && line . trim_end () . ends_with (" = {")) else { break ; } ; let block_ends = block_starts + block_ends ; let block_starts = block_starts + 1 ; let mut file_copy = file . clone () ; for _ in block_starts .. (block_ends) { file_copy . remove (block_starts) ; } file_copy . insert (block_starts , "Call(tmp = core::intrinsics::abort(), ReturnTo(bb1), UnwindUnreachable())\n" . to_string () ,) ; file_copy . insert (block_starts , "let tmp = ();\n" . to_string ()) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by {} lines `block_abort`" , block_ends - block_starts - 2) ; * file = file_copy ; curr = block_starts ; } else { curr = block_ends ; } } save_reduction (file , path , "block_abort") ; }
}

macro_rules! remove_block_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_block in module {}", module_path!());
    };
}

mkfn!{
    remove_block_introspect!();
    # [doc = " Removes unreachable basic blocks"] fn remove_block (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (block_starts) = file [curr ..] . iter () . position (| line | line . starts_with ("bb") && line . trim_end () . ends_with (" = {")) else { break ; } ; let block_starts = block_starts + curr ; let Some (block_ends) = file [(block_starts + 1) ..] . iter () . position (| line | line . starts_with ("bb") && line . trim_end () . ends_with (" = {")) else { break ; } ; let block_ends = block_starts + block_ends + 1 ; if block_ends - block_starts > 6 { curr = block_starts + 1 ; continue ; } let mut file_copy = file . clone () ; file_copy . drain (block_starts .. block_ends) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by {} lines `remove_blocks`" , block_ends - block_starts) ; * file = file_copy ; curr = block_starts ; } else { curr = block_starts + 1 ; } } save_reduction (file , path , "remove_block") ; }
}

macro_rules! linearize_cf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linearize_cf in module {}", module_path!());
    };
}

mkfn!{
    linearize_cf_introspect!();
    # [doc = " Merges blocks ending with unconditional jumps."] fn linearize_cf (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (block_starts) = file [curr ..] . iter () . position (| line | line . starts_with ("bb") && line . trim_end () . ends_with (" = {")) else { break ; } ; let block_starts = block_starts + curr ; let Some ((block , _)) = file [block_starts] . split_once ('=') else { curr = block_starts + 1 ; continue ; } ; let block = block . trim () ; if file [block_starts - 2] . trim () != format ! ("Goto({block})") { curr = block_starts + 1 ; continue ; } let mut file_copy = file . clone () ; file_copy . remove (block_starts - 2) ; file_copy . remove (block_starts - 2) ; file_copy . remove (block_starts - 2) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by 3 lines `linearize_cf`") ; * file = file_copy ; curr = block_starts ; } else { curr = block_starts + 1 ; } } save_reduction (file , path , "linearize_cf") ; }
}

macro_rules! remove_fn_calls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_fn_calls in module {}", module_path!());
    };
}

mkfn!{
    remove_fn_calls_introspect!();
    # [doc = " Replaces a call to a given function with a 0 assignment to the destination place, and a Goto."] # [doc = " This is always sound, because:"] # [doc = " 1. All the functions arguments are always initialized"] # [doc = " 2. and point to initialized  memory(the operand of &raw must be an initialized place in rustlantis)."] fn remove_fn_calls (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (fn_call) = file [curr ..] . iter () . position (| line | line . contains ("Call(") && line . contains (" = fn")) else { break ; } ; let fn_call = fn_call + curr ; let line = file [fn_call] . trim () ; let line = & line ["Call(" . len () ..] ; let Some ((place , line)) = line . split_once ('=') else { curr = fn_call + 1 ; continue ; } ; let Some ((_ , line)) = line . split_once ("ReturnTo(") else { curr = fn_call + 1 ; continue ; } ; let Some ((block , _)) = line . split_once (')') else { curr = fn_call + 1 ; continue ; } ; let mut file_copy = file . clone () ; file_copy . remove (fn_call) ; file_copy . insert (fn_call , format ! ("Goto({block})\n")) ; file_copy . insert (fn_call , format ! ("{place} = 0;\n")) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} using `remove_fn_calls` {cache:?}") ; * file = file_copy ; curr = fn_call ; } else { curr = fn_call + 1 ; } } save_reduction (file , path , "remove_fn_calls") ; }
}

macro_rules! remove_fns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_fns in module {}", module_path!());
    };
}

mkfn!{
    remove_fns_introspect!();
    # [doc = " Fully removes unreachable functions."] fn remove_fns (file : & mut Vec < String > , path : & PathBuf , cache : & mut ResultCache) { let mut curr = 0 ; while curr < file . len () { let Some (fn_start) = file [curr ..] . iter () . position (| line | { line . contains ("#[custom_mir(dialect = \"runtime\", phase = \"initial\")]") }) else { break ; } ; let fn_start = fn_start + curr ; let Some (fn_end) = file [(fn_start + 3) ..] . iter () . position (| line | line . contains ("fn fn")) else { break ; } ; let fn_end = fn_start + 2 + fn_end ; let mut file_copy = file . clone () ; file_copy . drain (fn_start .. fn_end) ; if test_reduction (& file_copy , path , cache) { println ! ("Reduced {path:?} by {} lines `remove_fns`" , fn_end - fn_start) ; * file = file_copy ; } else { curr = fn_start + 1 ; } } save_reduction (file , path , "remove_fns") ; }
}

macro_rules! reduce_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reduce in module {}", module_path!());
    };
}

mkfn!{
    reduce_introspect!();
    pub (super) fn reduce (path : impl AsRef < Path >) { let path = path . as_ref () . to_owned () ; let file = std :: fs :: read_to_string (& path) . expect ("Could not open the file to reduce") ; let mut file : Vec < _ > = file . split_inclusive ('\n') . map (| s | s . to_string ()) . collect () ; println ! ("running `remove_dump_var` on {path:?}.") ; remove_dump_var (& mut file , & path) ; let mut cache = None ; assert ! (test_reduction (& file , & path , & mut cache) , "Reduction error: check that the input file is a valid reproducer.") ; println ! ("cache:{cache:?}") ; println ! ("running `remove_fn_calls` on {path:?}.") ; remove_fn_calls (& mut file , & path , & mut cache) ; println ! ("running `remove_fns` on {path:?}.") ; remove_fns (& mut file , & path , & mut cache) ; let len = file . len () ; println ! ("running `remove_dup_assign` on {path:?}.") ; remove_dup_assign (& mut file , & path , 0 , len , & mut cache) ; file . retain (| line | ! line . is_empty ()) ; println ! ("running `match_to_goto` on {path:?}.") ; match_to_goto (& mut file , & path , & mut cache) ; println ! ("running `block_abort` on {path:?}.") ; block_abort (& mut file , & path , & mut cache) ; println ! ("running `remove_block` on {path:?}.") ; remove_block (& mut file , & path , & mut cache) ; println ! ("running `linearize_cf` on {path:?}.") ; linearize_cf (& mut file , & path , & mut cache) ; let mut out = std :: fs :: File :: create (& path) . expect ("Could not save the reduction result.") ; let file = file . into_iter () . collect :: < String > () ; out . write_all (file . as_bytes ()) . expect ("failed to write into file") ; }
}