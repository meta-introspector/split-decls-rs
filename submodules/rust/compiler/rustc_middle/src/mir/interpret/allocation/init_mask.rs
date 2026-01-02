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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
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
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use std :: ops :: Range ;}
mkuse!{use std :: { hash , iter } ;}
mkuse!{use rustc_abi :: Size ;}
mkuse!{use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable } ;}
mkuse!{use rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkuse!{use super :: AllocRange ;}
mkitem!{type Block = u64 ;}
mkitem!{mkstruct!{#[doc = " A bitmask where each bit refers to the byte with the same index. If the bit is `true`, the byte"] #[doc = " is initialized. If it is `false` the byte is uninitialized."] #[doc = " The actual bits are only materialized when needed, and we try to keep this data lazy as long as"] #[doc = " possible. Currently, if all the blocks have the same value, then the mask represents either a"] #[doc = " fully initialized or fully uninitialized const allocation, so we can only store that single"] #[doc = " value."] #[derive (Clone , Debug , Eq , PartialEq , Encodable_NoContext , Decodable_NoContext , Hash , HashStable)] pub struct InitMask { blocks : InitMaskBlocks , len : Size , }}}
mkitem!{mkenum!{#[derive (Clone , Debug , Eq , PartialEq , Encodable_NoContext , Decodable_NoContext , Hash , HashStable)] enum InitMaskBlocks { Lazy { #[doc = " Whether the lazy init mask is fully initialized or uninitialized."] state : bool , } , Materialized (InitMaskMaterialized) , }}}
mkitem!{mkimpl!{impl InitMask { pub fn new (size : Size , state : bool) -> Self { let blocks = InitMaskBlocks :: Lazy { state } ; InitMask { len : size , blocks } } #[doc = " Checks whether the `range` is entirely initialized."] #[doc = ""] #[doc = " Returns `Ok(())` if it's initialized. Otherwise returns a range of byte"] #[doc = " indexes for the first contiguous span of the uninitialized access."] #[inline] pub fn is_range_initialized (& self , range : AllocRange) -> Result < () , AllocRange > { let end = range . end () ; if end > self . len { return Err (AllocRange :: from (self . len .. end)) ; } match self . blocks { InitMaskBlocks :: Lazy { state } => { if state { Ok (()) } else { Err (range) } } InitMaskBlocks :: Materialized (ref blocks) => { blocks . is_range_initialized (range . start , end) } } } #[doc = " Sets a specified range to a value. If the range is out-of-bounds, the mask will grow to"] #[doc = " accommodate it entirely."] pub fn set_range (& mut self , range : AllocRange , new_state : bool) { let start = range . start ; let end = range . end () ; let is_full_overwrite = start == Size :: ZERO && end >= self . len ; match self . blocks { InitMaskBlocks :: Lazy { ref mut state } if is_full_overwrite => { * state = new_state ; self . len = end ; } InitMaskBlocks :: Materialized (_) if is_full_overwrite => { self . blocks = InitMaskBlocks :: Lazy { state : new_state } ; self . len = end ; } InitMaskBlocks :: Lazy { state } if state == new_state => { if end > self . len { self . len = end ; } } _ => { let len = self . len ; let blocks = self . materialize_blocks () ; if end <= len { blocks . set_range_inbounds (start , end , new_state) ; } else { if start < len { blocks . set_range_inbounds (start , len , new_state) ; } blocks . grow (len , end - len , new_state) ; self . len = end ; } } } } #[doc = " Materializes this mask's blocks when the mask is lazy."] #[inline] fn materialize_blocks (& mut self) -> & mut InitMaskMaterialized { if let InitMaskBlocks :: Lazy { state } = self . blocks { self . blocks = InitMaskBlocks :: Materialized (InitMaskMaterialized :: new (self . len , state)) ; } let InitMaskBlocks :: Materialized (ref mut blocks) = self . blocks else { bug ! ("initmask blocks must be materialized here") } ; blocks } #[doc = " Returns the initialization state at the specified in-bounds index."] #[inline] pub fn get (& self , idx : Size) -> bool { match self . blocks { InitMaskBlocks :: Lazy { state } => state , InitMaskBlocks :: Materialized (ref blocks) => blocks . get (idx) , } } }}}
mkitem!{mkstruct!{#[doc = " The actual materialized blocks of the bitmask, when we can't keep the `InitMask` lazy."] #[derive (Clone , Debug , Eq , PartialEq , HashStable)] struct InitMaskMaterialized { blocks : Vec < Block > , }}}
mkitem!{mkimpl!{impl < E : Encoder > Encodable < E > for InitMaskMaterialized { fn encode (& self , encoder : & mut E) { encoder . emit_usize (self . blocks . len ()) ; for block in & self . blocks { encoder . emit_raw_bytes (& block . to_le_bytes ()) ; } } }}}
mkitem!{mkimpl!{impl < D : Decoder > Decodable < D > for InitMaskMaterialized { fn decode (decoder : & mut D) -> Self { let num_blocks = decoder . read_usize () ; let mut blocks = Vec :: with_capacity (num_blocks) ; for _ in 0 .. num_blocks { let bytes = decoder . read_raw_bytes (8) ; let block = u64 :: from_le_bytes (bytes . try_into () . unwrap ()) ; blocks . push (block) ; } InitMaskMaterialized { blocks } } }}}
mkitem!{mkimpl!{impl hash :: Hash for InitMaskMaterialized { fn hash < H : hash :: Hasher > (& self , state : & mut H) { const MAX_BLOCKS_TO_HASH : usize = super :: MAX_BYTES_TO_HASH / size_of :: < Block > () ; const MAX_BLOCKS_LEN : usize = super :: MAX_HASHED_BUFFER_LEN / size_of :: < Block > () ; let block_count = self . blocks . len () ; if block_count > MAX_BLOCKS_LEN { block_count . hash (state) ; self . blocks [.. MAX_BLOCKS_TO_HASH] . hash (state) ; self . blocks [block_count - MAX_BLOCKS_TO_HASH ..] . hash (state) ; } else { self . blocks . hash (state) ; } } }}}
mkitem!{mkimpl!{impl InitMaskMaterialized { const BLOCK_SIZE : u64 = 64 ; fn new (size : Size , state : bool) -> Self { let mut m = InitMaskMaterialized { blocks : vec ! [] } ; m . grow (Size :: ZERO , size , state) ; m } #[inline] fn bit_index (bits : Size) -> (usize , usize) { let bits = bits . bytes () ; let a = bits / Self :: BLOCK_SIZE ; let b = bits % Self :: BLOCK_SIZE ; (usize :: try_from (a) . unwrap () , usize :: try_from (b) . unwrap ()) } #[inline] fn size_from_bit_index (block : impl TryInto < u64 > , bit : impl TryInto < u64 >) -> Size { let block = block . try_into () . ok () . unwrap () ; let bit = bit . try_into () . ok () . unwrap () ; Size :: from_bytes (block * Self :: BLOCK_SIZE + bit) } #[doc = " Checks whether the `range` is entirely initialized."] #[doc = ""] #[doc = " Returns `Ok(())` if it's initialized. Otherwise returns a range of byte"] #[doc = " indexes for the first contiguous span of the uninitialized access."] #[inline] fn is_range_initialized (& self , start : Size , end : Size) -> Result < () , AllocRange > { let uninit_start = self . find_bit (start , end , false) ; match uninit_start { Some (uninit_start) => { let uninit_end = self . find_bit (uninit_start , end , true) . unwrap_or (end) ; Err (AllocRange :: from (uninit_start .. uninit_end)) } None => Ok (()) , } } fn set_range_inbounds (& mut self , start : Size , end : Size , new_state : bool) { let (block_a , bit_a) = Self :: bit_index (start) ; let (block_b , bit_b) = Self :: bit_index (end) ; if block_a == block_b { let range = if bit_b == 0 { u64 :: MAX << bit_a } else { (u64 :: MAX << bit_a) & (u64 :: MAX >> (64 - bit_b)) } ; if new_state { self . blocks [block_a] |= range ; } else { self . blocks [block_a] &= ! range ; } return ; } if new_state { self . blocks [block_a] |= u64 :: MAX << bit_a ; if bit_b != 0 { self . blocks [block_b] |= u64 :: MAX >> (64 - bit_b) ; } for block in (block_a + 1) .. block_b { self . blocks [block] = u64 :: MAX ; } } else { self . blocks [block_a] &= ! (u64 :: MAX << bit_a) ; if bit_b != 0 { self . blocks [block_b] &= ! (u64 :: MAX >> (64 - bit_b)) ; } for block in (block_a + 1) .. block_b { self . blocks [block] = 0 ; } } } #[inline] fn get (& self , i : Size) -> bool { let (block , bit) = Self :: bit_index (i) ; (self . blocks [block] & (1 << bit)) != 0 } fn grow (& mut self , len : Size , amount : Size , new_state : bool) { if amount . bytes () == 0 { return ; } let unused_trailing_bits = u64 :: try_from (self . blocks . len ()) . unwrap () * Self :: BLOCK_SIZE - len . bytes () ; if amount . bytes () > unused_trailing_bits { let additional_blocks = amount . bytes () / Self :: BLOCK_SIZE + 1 ; let block = if new_state { u64 :: MAX } else { 0 } ; self . blocks . extend (iter :: repeat (block) . take (usize :: try_from (additional_blocks) . unwrap ())) ; } if unused_trailing_bits > 0 { let in_bounds_tail = Size :: from_bytes (unused_trailing_bits) ; self . set_range_inbounds (len , len + in_bounds_tail , new_state) ; } } #[doc = " Returns the index of the first bit in `start..end` (end-exclusive) that is equal to is_init."] fn find_bit (& self , start : Size , end : Size , is_init : bool) -> Option < Size > { #[doc = " A fast implementation of `find_bit`,"] #[doc = " which skips over an entire block at a time if it's all 0s (resp. 1s),"] #[doc = " and finds the first 1 (resp. 0) bit inside a block using `trailing_zeros` instead of a loop."] #[doc = ""] #[doc = " Note that all examples below are written with 8 (instead of 64) bit blocks for simplicity,"] #[doc = " and with the least significant bit (and lowest block) first:"] #[doc = " ```text"] #[doc = "        00000000|00000000"] #[doc = "        ^      ^ ^      ^"] #[doc = " index: 0      7 8      15"] #[doc = " ```"] #[doc = " Also, if not stated, assume that `is_init = true`, that is, we are searching for the first 1 bit."] fn find_bit_fast (init_mask : & InitMaskMaterialized , start : Size , end : Size , is_init : bool ,) -> Option < Size > { #[doc = " Search one block, returning the index of the first bit equal to `is_init`."] fn search_block (bits : Block , block : usize , start_bit : usize , is_init : bool ,) -> Option < Size > { let bits = if is_init { bits } else { ! bits } ; let bits = bits & (! 0 << start_bit) ; if bits == 0 { None } else { let bit = bits . trailing_zeros () ; Some (InitMaskMaterialized :: size_from_bit_index (block , bit)) } } if start >= end { return None ; } let (start_block , start_bit) = InitMaskMaterialized :: bit_index (start) ; let end_inclusive = Size :: from_bytes (end . bytes () - 1) ; let (end_block_inclusive , _) = InitMaskMaterialized :: bit_index (end_inclusive) ; if let Some (i) = search_block (init_mask . blocks [start_block] , start_block , start_bit , is_init) { if i < end { return Some (i) ; } else { return None ; } } if start_block < end_block_inclusive { for (& bits , block) in init_mask . blocks [start_block + 1 .. end_block_inclusive + 1] . iter () . zip (start_block + 1 ..) { if let Some (i) = search_block (bits , block , 0 , is_init) { if i < end { return Some (i) ; } else { return None ; } } } } None } #[cfg_attr (not (debug_assertions) , allow (dead_code))] fn find_bit_slow (init_mask : & InitMaskMaterialized , start : Size , end : Size , is_init : bool ,) -> Option < Size > { (start .. end) . find (| & i | init_mask . get (i) == is_init) } let result = find_bit_fast (self , start , end , is_init) ; debug_assert_eq ! (result , find_bit_slow (self , start , end , is_init) , "optimized implementation of find_bit is wrong for start={start:?} end={end:?} is_init={is_init} init_mask={self:#?}") ; result } }}}
mkitem!{mkenum!{#[doc = " A contiguous chunk of initialized or uninitialized memory."] pub enum InitChunk { Init (Range < Size >) , Uninit (Range < Size >) , }}}
mkitem!{mkimpl!{impl InitChunk { #[inline] pub fn is_init (& self) -> bool { match self { Self :: Init (_) => true , Self :: Uninit (_) => false , } } #[inline] pub fn range (& self) -> Range < Size > { match self { Self :: Init (r) => r . clone () , Self :: Uninit (r) => r . clone () , } } }}}
mkitem!{mkimpl!{impl InitMask { #[doc = " Returns an iterator, yielding a range of byte indexes for each contiguous region"] #[doc = " of initialized or uninitialized bytes inside the range `start..end` (end-exclusive)."] #[doc = ""] #[doc = " The iterator guarantees the following:"] #[doc = " - Chunks are nonempty."] #[doc = " - Chunks are adjacent (each range's start is equal to the previous range's end)."] #[doc = " - Chunks span exactly `start..end` (the first starts at `start`, the last ends at `end`)."] #[doc = " - Chunks alternate between [`InitChunk::Init`] and [`InitChunk::Uninit`]."] #[inline] pub fn range_as_init_chunks (& self , range : AllocRange) -> InitChunkIter < '_ > { let start = range . start ; let end = range . end () ; assert ! (end <= self . len) ; let is_init = if start < end { self . get (start) } else { false } ; InitChunkIter { init_mask : self , is_init , start , end } } }}}
mkitem!{mkstruct!{#[doc = " Yields [`InitChunk`]s. See [`InitMask::range_as_init_chunks`]."] #[derive (Clone)] pub struct InitChunkIter < 'a > { init_mask : & 'a InitMask , #[doc = " Whether the next chunk we will return is initialized."] #[doc = " If there are no more chunks, contains some arbitrary value."] is_init : bool , #[doc = " The current byte index into `init_mask`."] start : Size , #[doc = " The end byte index into `init_mask`."] end : Size , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for InitChunkIter < 'a > { type Item = InitChunk ; #[inline] fn next (& mut self) -> Option < Self :: Item > { if self . start >= self . end { return None ; } let end_of_chunk = match self . init_mask . blocks { InitMaskBlocks :: Lazy { .. } => { self . end } InitMaskBlocks :: Materialized (ref blocks) => { let end_of_chunk = blocks . find_bit (self . start , self . end , ! self . is_init) . unwrap_or (self . end) ; end_of_chunk } } ; let range = self . start .. end_of_chunk ; let ret = Some (if self . is_init { InitChunk :: Init (range) } else { InitChunk :: Uninit (range) }) ; self . is_init = ! self . is_init ; self . start = end_of_chunk ; ret } }}}
mkitem!{mkstruct!{#[doc = " Run-length encoding of the uninit mask."] #[doc = " Used to copy parts of a mask multiple times to another allocation."] pub struct InitCopy { #[doc = " Whether the first range is initialized."] initial : bool , #[doc = " The lengths of ranges that are run-length encoded."] #[doc = " The initialization state of the ranges alternate starting with `initial`."] ranges : smallvec :: SmallVec < [u64 ; 1] > , }}}
mkitem!{mkimpl!{impl InitCopy { pub fn no_bytes_init (& self) -> bool { ! self . initial && self . ranges . len () == 1 } }}}
mkitem!{mkimpl!{#[doc = " Transferring the initialization mask to other allocations."] impl InitMask { #[doc = " Creates a run-length encoding of the initialization mask; panics if range is empty."] #[doc = ""] #[doc = " This is essentially a more space-efficient version of"] #[doc = " `InitMask::range_as_init_chunks(...).collect::<Vec<_>>()`."] pub fn prepare_copy (& self , range : AllocRange) -> InitCopy { let mut ranges = smallvec :: SmallVec :: < [u64 ; 1] > :: new () ; let mut chunks = self . range_as_init_chunks (range) . peekable () ; let initial = chunks . peek () . expect ("range should be nonempty") . is_init () ; for chunk in chunks { let len = chunk . range () . end . bytes () - chunk . range () . start . bytes () ; ranges . push (len) ; } InitCopy { ranges , initial } } #[doc = " Applies multiple instances of the run-length encoding to the initialization mask."] pub fn apply_copy (& mut self , defined : InitCopy , range : AllocRange , repeat : u64) { if defined . ranges . len () <= 1 { let start = range . start ; let end = range . start + range . size * repeat ; self . set_range (AllocRange :: from (start .. end) , defined . initial) ; return ; } let blocks = self . materialize_blocks () ; for mut j in 0 .. repeat { j *= range . size . bytes () ; j += range . start . bytes () ; let mut cur = defined . initial ; for range in & defined . ranges { let old_j = j ; j += range ; blocks . set_range_inbounds (Size :: from_bytes (old_j) , Size :: from_bytes (j) , cur) ; cur = ! cur ; } } } }}}