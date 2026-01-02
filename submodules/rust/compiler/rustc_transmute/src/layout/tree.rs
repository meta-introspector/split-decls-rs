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
mkuse!{use std :: ops :: { ControlFlow , RangeInclusive } ;}
mkuse!{use super :: { Byte , Def , Reference , Region , Type } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkenum!{#[doc = " A tree-based representation of a type layout."] #[doc = ""] #[doc = " Invariants:"] #[doc = " 1. All paths through the layout have the same length (in bytes)."] #[doc = ""] #[doc = " Nice-to-haves:"] #[doc = " 1. An `Alt` is never directly nested beneath another `Alt`."] #[doc = " 2. A `Seq` is never directly nested beneath another `Seq`."] #[doc = " 3. `Seq`s and `Alt`s with a single member do not exist."] #[derive (Clone , Debug , Hash , PartialEq , Eq)] pub (crate) enum Tree < D , R , T > where D : Def , R : Region , T : Type , { #[doc = " A sequence of successive layouts."] Seq (Vec < Self >) , #[doc = " A choice between alternative layouts."] Alt (Vec < Self >) , #[doc = " A definition node."] Def (D) , #[doc = " A reference node."] Ref (Reference < R , T >) , #[doc = " A byte node."] Byte (Byte) , }}}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) enum Endian { Little , Big , }}}
mkitem!{mkimpl!{#[cfg (feature = "rustc")] impl From < rustc_abi :: Endian > for Endian { fn from (order : rustc_abi :: Endian) -> Endian { match order { rustc_abi :: Endian :: Little => Endian :: Little , rustc_abi :: Endian :: Big => Endian :: Big , } } }}}
mkitem!{mkimpl!{impl < D , R , T > Tree < D , R , T > where D : Def , R : Region , T : Type , { #[doc = " A `Tree` consisting only of a definition node."] pub (crate) fn def (def : D) -> Self { Self :: Def (def) } #[doc = " A `Tree` representing an uninhabited type."] pub (crate) fn uninhabited () -> Self { Self :: Alt (vec ! []) } #[doc = " A `Tree` representing a zero-sized type."] pub (crate) fn unit () -> Self { Self :: Seq (Vec :: new ()) } #[doc = " A `Tree` containing a single, uninitialized byte."] pub (crate) fn uninit () -> Self { Self :: Byte (Byte :: uninit ()) } #[doc = " A `Tree` representing the layout of `bool`."] pub (crate) fn bool () -> Self { Self :: byte (0x00 ..= 0x01) } #[doc = " A `Tree` whose layout matches that of a `u8`."] pub (crate) fn u8 () -> Self { Self :: byte (0x00 ..= 0xFF) } #[doc = " A `Tree` whose layout matches that of a `char`."] pub (crate) fn char (order : Endian) -> Self { const _0 : RangeInclusive < u8 > = 0 ..= 0 ; const BYTE : RangeInclusive < u8 > = 0x00 ..= 0xFF ; let x = Self :: from_big_endian (order , [_0 , _0 , 0x00 ..= 0xD7 , BYTE]) ; let y = Self :: from_big_endian (order , [_0 , _0 , 0xE0 ..= 0xFF , BYTE]) ; let z = Self :: from_big_endian (order , [_0 , 0x01 ..= 0x10 , BYTE , BYTE]) ; Self :: alt ([x , y , z]) } #[doc = " A `Tree` whose layout matches `std::num::NonZeroXxx`."] #[allow (dead_code)] pub (crate) fn nonzero (width_in_bytes : u64) -> Self { const BYTE : RangeInclusive < u8 > = 0x00 ..= 0xFF ; const NONZERO : RangeInclusive < u8 > = 0x01 ..= 0xFF ; (0 .. width_in_bytes) . map (| nz_idx | { (0 .. width_in_bytes) . map (| pos | Self :: byte (if pos == nz_idx { NONZERO } else { BYTE })) . fold (Self :: unit () , Self :: then) }) . fold (Self :: uninhabited () , Self :: or) } pub (crate) fn bytes < const N : usize , B : Into < Byte > > (bytes : [B ; N]) -> Self { Self :: seq (bytes . map (B :: into) . map (Self :: Byte)) } pub (crate) fn byte (byte : impl Into < Byte >) -> Self { Self :: Byte (byte . into ()) } #[doc = " A `Tree` whose layout is a number of the given width."] pub (crate) fn number (width_in_bytes : u64) -> Self { Self :: Seq (vec ! [Self :: u8 () ; width_in_bytes . try_into () . unwrap ()]) } #[doc = " A `Tree` whose layout is entirely padding of the given width."] pub (crate) fn padding (width_in_bytes : usize) -> Self { Self :: Seq (vec ! [Self :: uninit () ; width_in_bytes]) } #[doc = " Remove all `Def` nodes, and all branches of the layout for which `f`"] #[doc = " produces `true`."] pub (crate) fn prune < F > (self , f : & F) -> Tree < ! , R , T > where F : Fn (D) -> bool , { match self { Self :: Seq (elts) => match elts . into_iter () . map (| elt | elt . prune (f)) . try_fold (Tree :: unit () , | elts , elt | { if elt == Tree :: uninhabited () { ControlFlow :: Break (Tree :: uninhabited ()) } else { ControlFlow :: Continue (elts . then (elt)) } } ,) { ControlFlow :: Break (node) | ControlFlow :: Continue (node) => node , } , Self :: Alt (alts) => alts . into_iter () . map (| alt | alt . prune (f)) . fold (Tree :: uninhabited () , | alts , alt | alts . or (alt)) , Self :: Byte (b) => Tree :: Byte (b) , Self :: Ref (r) => Tree :: Ref (r) , Self :: Def (d) => { if f (d) { Tree :: uninhabited () } else { Tree :: unit () } } } } #[doc = " Produces `true` if `Tree` is an inhabited type; otherwise false."] pub (crate) fn is_inhabited (& self) -> bool { match self { Self :: Seq (elts) => elts . into_iter () . all (| elt | elt . is_inhabited ()) , Self :: Alt (alts) => alts . into_iter () . any (| alt | alt . is_inhabited ()) , Self :: Byte (..) | Self :: Ref (..) | Self :: Def (..) => true , } } #[doc = " Produces a `Tree` which represents a sequence of bytes stored in"] #[doc = " `order`."] #[doc = ""] #[doc = " `bytes` is taken to be in big-endian byte order, and its order will be"] #[doc = " swapped if `order == Endian::Little`."] pub (crate) fn from_big_endian < const N : usize , B : Into < Byte > > (order : Endian , mut bytes : [B ; N] ,) -> Self { if order == Endian :: Little { (& mut bytes [..]) . reverse () ; } Self :: bytes (bytes) } #[doc = " Produces a `Tree` where each of the trees in `trees` are sequenced one"] #[doc = " after another."] pub (crate) fn seq < const N : usize > (trees : [Tree < D , R , T > ; N]) -> Self { trees . into_iter () . fold (Tree :: unit () , Self :: then) } #[doc = " Produces a `Tree` where each of the trees in `trees` are accepted as"] #[doc = " alternative layouts."] pub (crate) fn alt < const N : usize > (trees : [Tree < D , R , T > ; N]) -> Self { trees . into_iter () . fold (Tree :: uninhabited () , Self :: or) } #[doc = " Produces a new `Tree` where `other` is sequenced after `self`."] pub (crate) fn then (self , other : Self) -> Self { match (self , other) { (Self :: Seq (elts) , other) | (other , Self :: Seq (elts)) if elts . len () == 0 => other , (Self :: Seq (mut lhs) , Self :: Seq (mut rhs)) => { lhs . append (& mut rhs) ; Self :: Seq (lhs) } (Self :: Seq (mut lhs) , rhs) => { lhs . push (rhs) ; Self :: Seq (lhs) } (lhs , Self :: Seq (mut rhs)) => { rhs . insert (0 , lhs) ; Self :: Seq (rhs) } (lhs , rhs) => Self :: Seq (vec ! [lhs , rhs]) , } } #[doc = " Produces a new `Tree` accepting either `self` or `other` as alternative layouts."] pub (crate) fn or (self , other : Self) -> Self { match (self , other) { (Self :: Alt (alts) , other) | (other , Self :: Alt (alts)) if alts . len () == 0 => other , (Self :: Alt (mut lhs) , Self :: Alt (rhs)) => { lhs . extend (rhs) ; Self :: Alt (lhs) } (Self :: Alt (mut alts) , alt) | (alt , Self :: Alt (mut alts)) => { alts . push (alt) ; Self :: Alt (alts) } (lhs , rhs) => Self :: Alt (vec ! [lhs , rhs]) , } } }}}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                mkuse!{use rustc_abi :: { FieldIdx , FieldsShape , Layout , Size , TagEncoding , TyAndLayout , VariantIdx , Variants , } ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , LayoutCx , LayoutError } ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , AdtKind , List , Region , ScalarInt , Ty , TyCtxt , TypeVisitableExt , } ;}
mkuse!{use rustc_span :: ErrorGuaranteed ;}
mkuse!{use super :: Tree ;}
mkuse!{use crate :: layout :: Reference ;}
mkuse!{use crate :: layout :: rustc :: { Def , layout_of } ;}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone)] pub (crate) enum Err { #[doc = " The layout of the type is not yet supported."] NotYetSupported , #[doc = " This error will be surfaced elsewhere by rustc, so don't surface it."] UnknownLayout , #[doc = " Overflow size"] SizeOverflow , TypeError (ErrorGuaranteed) , }}}
mkitem!{mkimpl!{impl < 'tcx > From < & LayoutError < 'tcx > > for Err { fn from (err : & LayoutError < 'tcx >) -> Self { match err { LayoutError :: Unknown (..) | LayoutError :: ReferencesError (..) | LayoutError :: TooGeneric (..) | LayoutError :: NormalizationFailure (..) => Self :: UnknownLayout , LayoutError :: SizeOverflow (..) => Self :: SizeOverflow , LayoutError :: Cycle (err) => Self :: TypeError (* err) , } } }}}
mkitem!{mkimpl!{impl < 'tcx > Tree < Def < 'tcx > , Region < 'tcx > , Ty < 'tcx > > { pub (crate) fn from_ty (ty : Ty < 'tcx > , cx : LayoutCx < 'tcx >) -> Result < Self , Err > { use rustc_abi :: HasDataLayout ; let layout = layout_of (cx , ty) ? ; if let Err (e) = ty . error_reported () { return Err (Err :: TypeError (e)) ; } let target = cx . data_layout () ; let pointer_size = target . pointer_size () ; match ty . kind () { ty :: Bool => Ok (Self :: bool ()) , ty :: Float (nty) => { let width = nty . bit_width () / 8 ; Ok (Self :: number (width . try_into () . unwrap ())) } ty :: Int (nty) => { let width = nty . normalize (pointer_size . bits () as _) . bit_width () . unwrap () / 8 ; Ok (Self :: number (width . try_into () . unwrap ())) } ty :: Uint (nty) => { let width = nty . normalize (pointer_size . bits () as _) . bit_width () . unwrap () / 8 ; Ok (Self :: number (width . try_into () . unwrap ())) } ty :: Tuple (members) => Self :: from_tuple ((ty , layout) , members , cx) , ty :: Array (inner_ty , _len) => { let FieldsShape :: Array { stride , count } = & layout . fields else { return Err (Err :: NotYetSupported) ; } ; let inner_layout = layout_of (cx , * inner_ty) ? ; assert_eq ! (* stride , inner_layout . size) ; let elt = Tree :: from_ty (* inner_ty , cx) ? ; Ok (std :: iter :: repeat (elt) . take (* count as usize) . fold (Tree :: unit () , | tree , elt | tree . then (elt))) } ty :: Adt (adt_def , _args_ref) if ! ty . is_box () => { let (lo , hi) = cx . tcx () . layout_scalar_valid_range (adt_def . did ()) ; use core :: ops :: Bound :: * ; let is_transparent = adt_def . repr () . transparent () ; match (adt_def . adt_kind () , lo , hi) { (AdtKind :: Struct , Unbounded , Unbounded) => { Self :: from_struct ((ty , layout) , * adt_def , cx) } (AdtKind :: Struct , Included (1) , Included (_hi)) if is_transparent => { Err (Err :: NotYetSupported) } (AdtKind :: Enum , Unbounded , Unbounded) => { Self :: from_enum ((ty , layout) , * adt_def , cx) } (AdtKind :: Union , Unbounded , Unbounded) => { Self :: from_union ((ty , layout) , * adt_def , cx) } _ => Err (Err :: NotYetSupported) , } } ty :: Ref (region , ty , mutability) => { let layout = layout_of (cx , * ty) ? ; let referent_align = layout . align . abi . bytes_usize () ; let referent_size = layout . size . bytes_usize () ; Ok (Tree :: Ref (Reference { region : * region , is_mut : mutability . is_mut () , referent : * ty , referent_align , referent_size , })) } ty :: Char => Ok (Self :: char (cx . tcx () . data_layout . endian . into ())) , _ => Err (Err :: NotYetSupported) , } } #[doc = " Constructs a `Tree` from a tuple."] fn from_tuple ((ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , members : & 'tcx List < Ty < 'tcx > > , cx : LayoutCx < 'tcx > ,) -> Result < Self , Err > { match & layout . fields { FieldsShape :: Primitive => { assert_eq ! (members . len () , 1) ; let inner_ty = members [0] ; Self :: from_ty (inner_ty , cx) } FieldsShape :: Arbitrary { offsets , .. } => { assert_eq ! (offsets . len () , members . len ()) ; Self :: from_variant (Def :: Primitive , None , (ty , layout) , layout . size , cx) } FieldsShape :: Array { .. } | FieldsShape :: Union (_) => Err (Err :: NotYetSupported) , } } #[doc = " Constructs a `Tree` from a struct."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " Panics if `def` is not a struct definition."] fn from_struct ((ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , def : AdtDef < 'tcx > , cx : LayoutCx < 'tcx > ,) -> Result < Self , Err > { assert ! (def . is_struct ()) ; let def = Def :: Adt (def) ; Self :: from_variant (def , None , (ty , layout) , layout . size , cx) } #[doc = " Constructs a `Tree` from an enum."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " Panics if `def` is not an enum definition."] fn from_enum ((ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , def : AdtDef < 'tcx > , cx : LayoutCx < 'tcx > ,) -> Result < Self , Err > { assert ! (def . is_enum ()) ; let layout_of_variant = | index , encoding : Option < _ > | -> Result < Self , Err > { let variant_layout = ty_variant (cx , (ty , layout) , index) ; if variant_layout . is_uninhabited () { return Ok (Self :: uninhabited ()) ; } let tag = cx . tcx () . tag_for_variant (cx . typing_env . as_query_input ((cx . tcx () . erase_and_anonymize_regions (ty) , index)) ,) ; let variant_def = Def :: Variant (def . variant (index)) ; Self :: from_variant (variant_def , tag . map (| tag | (tag , index , encoding . unwrap ())) , (ty , variant_layout) , layout . size , cx ,) } ; match layout . variants () { Variants :: Empty => Ok (Self :: uninhabited ()) , Variants :: Single { index } => { layout_of_variant (* index , None) } Variants :: Multiple { tag : _ , tag_encoding , tag_field , .. } => { assert_eq ! (* tag_field , FieldIdx :: ZERO) ; let variants = def . discriminants (cx . tcx ()) . try_fold (Self :: uninhabited () , | variants , (idx , _discriminant) | { let variant = layout_of_variant (idx , Some (tag_encoding . clone ())) ? ; Result :: < Self , Err > :: Ok (variants . or (variant)) } ,) ? ; Ok (Self :: def (Def :: Adt (def)) . then (variants)) } } } #[doc = " Constructs a `Tree` from a 'variant-like' layout."] #[doc = ""] #[doc = " A 'variant-like' layout includes those of structs and, of course,"] #[doc = " enum variants. Pragmatically speaking, this method supports anything"] #[doc = " with `FieldsShape::Arbitrary`."] #[doc = ""] #[doc = " Note: This routine assumes that the optional `tag` is the first"] #[doc = " field, and enum callers should check that `tag_field` is, in fact,"] #[doc = " `0`."] fn from_variant (def : Def < 'tcx > , tag : Option < (ScalarInt , VariantIdx , TagEncoding < VariantIdx >) > , (ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , total_size : Size , cx : LayoutCx < 'tcx > ,) -> Result < Self , Err > { let FieldsShape :: Arbitrary { offsets , memory_index } = layout . fields () else { return Err (Err :: NotYetSupported) ; } ; assert ! (layout . size <= total_size) ; let mut size = Size :: ZERO ; let mut struct_tree = Self :: def (def) ; if let Some ((tag , index , encoding)) = & tag { match encoding { TagEncoding :: Direct => { size += tag . size () ; } TagEncoding :: Niche { niche_variants , .. } => { if ! niche_variants . contains (index) { size += tag . size () ; } } } struct_tree = struct_tree . then (Self :: from_tag (* tag , cx . tcx ())) ; } let inverse_memory_index = memory_index . invert_bijective_mapping () ; for & field_idx in inverse_memory_index . iter () { let padding_needed = offsets [field_idx] - size ; let padding = Self :: padding (padding_needed . bytes_usize ()) ; let field_ty = ty_field (cx , (ty , layout) , field_idx) ; let field_layout = layout_of (cx , field_ty) ? ; let field_tree = Self :: from_ty (field_ty , cx) ? ; struct_tree = struct_tree . then (padding) . then (field_tree) ; size += padding_needed + field_layout . size ; } let padding_needed = total_size - size ; let trailing_padding = Self :: padding (padding_needed . bytes_usize ()) ; Ok (struct_tree . then (trailing_padding)) } #[doc = " Constructs a `Tree` representing the value of a enum tag."] fn from_tag (tag : ScalarInt , tcx : TyCtxt < 'tcx >) -> Self { use rustc_abi :: Endian ; let size = tag . size () ; let bits = tag . to_bits (size) ; let bytes : [u8 ; 16] ; let bytes = match tcx . data_layout . endian { Endian :: Little => { bytes = bits . to_le_bytes () ; & bytes [.. size . bytes_usize ()] } Endian :: Big => { bytes = bits . to_be_bytes () ; & bytes [bytes . len () - size . bytes_usize () ..] } } ; Self :: Seq (bytes . iter () . map (| & b | Self :: byte (b)) . collect ()) } #[doc = " Constructs a `Tree` from a union."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc = " Panics if `def` is not a union definition."] fn from_union ((ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , def : AdtDef < 'tcx > , cx : LayoutCx < 'tcx > ,) -> Result < Self , Err > { assert ! (def . is_union ()) ; let FieldsShape :: Union (_fields) = layout . fields () else { return Err (Err :: NotYetSupported) ; } ; let fields = & def . non_enum_variant () . fields ; let fields = fields . iter_enumerated () . try_fold (Self :: uninhabited () , | fields , (idx , _field_def) | { let field_ty = ty_field (cx , (ty , layout) , idx) ; let field_layout = layout_of (cx , field_ty) ? ; let field = Self :: from_ty (field_ty , cx) ? ; let trailing_padding_needed = layout . size - field_layout . size ; let trailing_padding = Self :: padding (trailing_padding_needed . bytes_usize ()) ; let field_and_padding = field . then (trailing_padding) ; Result :: < Self , Err > :: Ok (fields . or (field_and_padding)) } ,) ? ; Ok (Self :: def (Def :: Adt (def)) . then (fields)) } }}}

macro_rules! ty_field_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ty_field in module {}", module_path!());
    };
}

mkfn!{
    ty_field_introspect!();
    fn ty_field < 'tcx > (cx : LayoutCx < 'tcx > , (ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , i : FieldIdx ,) -> Ty < 'tcx > { match ty . kind () { ty :: Adt (def , args) => { match layout . variants { Variants :: Single { index } => { let field = & def . variant (index) . fields [i] ; field . ty (cx . tcx () , args) } Variants :: Empty => panic ! ("there is no field in Variants::Empty types") , Variants :: Multiple { tag , .. } => { assert_eq ! (i . as_usize () , 0) ; ty :: layout :: PrimitiveExt :: to_ty (& tag . primitive () , cx . tcx ()) } } } ty :: Tuple (fields) => fields [i . as_usize ()] , kind => unimplemented ! ("only a subset of `Ty::ty_and_layout_field`'s functionality is implemented. implementation needed for {:?}" , kind) , } }
}

macro_rules! ty_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ty_variant in module {}", module_path!());
    };
}

mkfn!{
    ty_variant_introspect!();
    fn ty_variant < 'tcx > (cx : LayoutCx < 'tcx > , (ty , layout) : (Ty < 'tcx > , Layout < 'tcx >) , i : VariantIdx ,) -> Layout < 'tcx > { let ty = cx . tcx () . erase_and_anonymize_regions (ty) ; TyAndLayout { ty , layout } . for_variant (& cx , i) . layout }
} 
            }}