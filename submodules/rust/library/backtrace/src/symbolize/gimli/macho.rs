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
mkuse!{use super :: mystd :: path :: Path ;}
mkuse!{use super :: { Context , Endian , EndianSlice , Mapping , Stash , gimli } ;}
mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use alloc :: sync :: Arc ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: convert :: TryInto ;}
mkuse!{use object :: macho ;}
mkuse!{use object :: read :: macho :: { MachHeader , Nlist , Section , Segment as _ } ;}
mkuse!{use object :: { Bytes , NativeEndian } ;}
mkitem!{#[cfg (target_pointer_width = "32")] type Mach = object :: macho :: MachHeader32 < NativeEndian > ;}
mkitem!{#[cfg (target_pointer_width = "64")] type Mach = object :: macho :: MachHeader64 < NativeEndian > ;}
mkitem!{type MachSegment = < Mach as MachHeader > :: Segment ;}
mkitem!{type MachSection = < Mach as MachHeader > :: Section ;}
mkitem!{type MachNlist = < Mach as MachHeader > :: Nlist ;}
mkitem!{mkimpl!{impl Mapping { pub fn new (path : & Path) -> Option < Mapping > { let map = super :: mmap (path) ? ; let (macho , data) = find_header (& map) ? ; let endian = macho . endian () . ok () ? ; let uuid = macho . uuid (endian , data , 0) . ok () ? ; if let Some (uuid) = uuid { if let Some (parent) = path . parent () { if let Some (mapping) = Mapping :: load_dsym (parent , uuid) { return Some (mapping) ; } } } Mapping :: mk (map , | data , stash | { let (macho , data) = find_header (data) ? ; let endian = macho . endian () . ok () ? ; let obj = Object :: parse (macho , endian , data) ? ; Context :: new (stash , obj , None , None) }) } fn load_dsym (dir : & Path , uuid : [u8 ; 16]) -> Option < Mapping > { for entry in dir . read_dir () . ok () ? { let entry = entry . ok () ? ; let filename = match entry . file_name () . into_string () { Ok (name) => name , Err (_) => continue , } ; if ! filename . ends_with (".dSYM") { continue ; } let candidates = entry . path () . join ("Contents/Resources/DWARF") ; if let Some (mapping) = Mapping :: try_dsym_candidate (& candidates , uuid) { return Some (mapping) ; } } None } fn try_dsym_candidate (dir : & Path , uuid : [u8 ; 16]) -> Option < Mapping > { for entry in dir . read_dir () . ok () ? { let entry = entry . ok () ? ; let map = super :: mmap (& entry . path ()) ? ; let candidate = Mapping :: mk (map , | data , stash | { let (macho , data) = find_header (data) ? ; let endian = macho . endian () . ok () ? ; let entry_uuid = macho . uuid (endian , data , 0) . ok () ? ? ; if entry_uuid != uuid { return None ; } let obj = Object :: parse (macho , endian , data) ? ; Context :: new (stash , obj , None , None) }) ; if let Some (candidate) = candidate { return Some (candidate) ; } } None } }}}

macro_rules! find_header_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_header in module {}", module_path!());
    };
}

mkfn!{
    find_header_introspect!();
    fn find_header (data : & '_ [u8]) -> Option < (& '_ Mach , & '_ [u8]) > { use object :: endian :: BigEndian ; let desired_cpu = | | { if cfg ! (target_arch = "x86") { Some (macho :: CPU_TYPE_X86) } else if cfg ! (target_arch = "x86_64") { Some (macho :: CPU_TYPE_X86_64) } else if cfg ! (target_arch = "arm") { Some (macho :: CPU_TYPE_ARM) } else if cfg ! (target_arch = "aarch64") { Some (macho :: CPU_TYPE_ARM64) } else { None } } ; let mut data = Bytes (data) ; match data . clone () . read :: < object :: endian :: U32 < NativeEndian > > () . ok () ? . get (NativeEndian) { macho :: MH_MAGIC_64 | macho :: MH_CIGAM_64 | macho :: MH_MAGIC | macho :: MH_CIGAM => { } macho :: FAT_MAGIC | macho :: FAT_CIGAM => { let mut header_data = data ; let endian = BigEndian ; let header = header_data . read :: < macho :: FatHeader > () . ok () ? ; let nfat = header . nfat_arch . get (endian) ; let arch = (0 .. nfat) . filter_map (| _ | header_data . read :: < macho :: FatArch32 > () . ok ()) . find (| arch | desired_cpu () == Some (arch . cputype . get (endian))) ? ; let offset = arch . offset . get (endian) ; let size = arch . size . get (endian) ; data = data . read_bytes_at (offset . try_into () . ok () ? , size . try_into () . ok () ?) . ok () ? ; } macho :: FAT_MAGIC_64 | macho :: FAT_CIGAM_64 => { let mut header_data = data ; let endian = BigEndian ; let header = header_data . read :: < macho :: FatHeader > () . ok () ? ; let nfat = header . nfat_arch . get (endian) ; let arch = (0 .. nfat) . filter_map (| _ | header_data . read :: < macho :: FatArch64 > () . ok ()) . find (| arch | desired_cpu () == Some (arch . cputype . get (endian))) ? ; let offset = arch . offset . get (endian) ; let size = arch . size . get (endian) ; data = data . read_bytes_at (offset . try_into () . ok () ? , size . try_into () . ok () ?) . ok () ? ; } _ => return None , } Mach :: parse (data . 0 , 0) . ok () . map (| h | (h , data . 0)) }
}
mkitem!{mkstruct!{pub struct Object < 'a > { endian : NativeEndian , data : & 'a [u8] , dwarf : Option < & 'a [MachSection] > , syms : Vec < (& 'a [u8] , u64) > , syms_sort_by_name : bool , object_map : Option < object :: ObjectMap < 'a > > , object_mappings : Box < [Option < Option < Mapping > >] > , }}}
mkitem!{mkimpl!{impl < 'a > Object < 'a > { fn parse (mach : & 'a Mach , endian : NativeEndian , data : & 'a [u8]) -> Option < Object < 'a > > { let is_object = mach . filetype (endian) == object :: macho :: MH_OBJECT ; let mut dwarf = None ; let mut syms = Vec :: new () ; let mut syms_sort_by_name = false ; let mut commands = mach . load_commands (endian , data , 0) . ok () ? ; let mut object_map = None ; let mut object_mappings = Vec :: new () ; while let Ok (Some (command)) = commands . next () { if let Some ((segment , section_data)) = MachSegment :: from_command (command) . ok () ? { if segment . name () == b"__DWARF" || (is_object && segment . name () == b"") { dwarf = segment . sections (endian , section_data) . ok () ; } } else if let Some (symtab) = command . symtab () . ok () ? { let symbols = symtab . symbols :: < Mach , _ > (endian , data) . ok () ? ; syms = symbols . iter () . filter_map (| nlist : & MachNlist | { let name = nlist . name (endian , symbols . strings ()) . ok () ? ; if name . len () > 0 && nlist . is_definition () { Some ((name , u64 :: from (nlist . n_value (endian)))) } else { None } }) . collect () ; if is_object { syms . sort_unstable_by_key (| (name , _) | * name) ; syms_sort_by_name = true ; } else { syms . sort_unstable_by_key (| (_ , addr) | * addr) ; let map = symbols . object_map (endian) ; object_mappings . resize_with (map . objects () . len () , | | None) ; object_map = Some (map) ; } } } Some (Object { endian , data , dwarf , syms , syms_sort_by_name , object_map , object_mappings : object_mappings . into_boxed_slice () , }) } pub fn section (& self , _ : & Stash , name : & str) -> Option < & 'a [u8] > { let name = name . as_bytes () ; let dwarf = self . dwarf ? ; let section = dwarf . into_iter () . find (| section | { let section_name = section . name () ; section_name == name || { section_name . starts_with (b"__") && name . starts_with (b".") && & section_name [2 ..] == & name [1 ..] } }) ? ; Some (section . data (self . endian , self . data) . ok () ?) } pub fn search_symtab < 'b > (& 'b self , addr : u64) -> Option < & 'b [u8] > { debug_assert ! (! self . syms_sort_by_name) ; let i = match self . syms . binary_search_by_key (& addr , | (_ , addr) | * addr) { Ok (i) => i , Err (i) => i . checked_sub (1) ? , } ; let (sym , _addr) = self . syms . get (i) ? ; Some (sym) } #[doc = " Try to load a context for an object file."] #[doc = ""] #[doc = " If dsymutil was not run, then the DWARF may be found in the source object files."] pub (super) fn search_object_map < 'b > (& 'b mut self , addr : u64) -> Option < (& 'b Context < 'b > , u64) > { let object_map = self . object_map . as_ref () ? ; let symbol = object_map . get (addr) ? ; let object_index = symbol . object_index () ; let mapping = self . object_mappings . get_mut (object_index) ? ; if mapping . is_none () { * mapping = Some (object_mapping (object_map . objects () . get (object_index) ?)) ; } let cx : & 'b Context < 'static > = & mapping . as_ref () ? . as_ref () ? . cx ; let cx = unsafe { core :: mem :: transmute :: < & 'b Context < 'static > , & 'b Context < 'b > > (cx) } ; debug_assert ! (cx . object . syms . is_empty () || cx . object . syms_sort_by_name) ; let i = cx . object . syms . binary_search_by_key (& symbol . name () , | (name , _) | * name) . ok () ? ; let object_symbol = cx . object . syms . get (i) ? ; let object_addr = addr . wrapping_sub (symbol . address ()) . wrapping_add (object_symbol . 1) ; Some ((cx , object_addr)) } }}}

macro_rules! object_mapping_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function object_mapping in module {}", module_path!());
    };
}

mkfn!{
    object_mapping_introspect!();
    fn object_mapping (file : & object :: read :: ObjectMapFile < '_ >) -> Option < Mapping > { use super :: mystd :: ffi :: OsStr ; use super :: mystd :: os :: unix :: prelude :: * ; let map = super :: mmap (Path :: new (OsStr :: from_bytes (file . path ()))) ? ; let member_name = file . member () ; Mapping :: mk (map , | data , stash | { let data = match member_name { Some (member_name) => { let archive = object :: read :: archive :: ArchiveFile :: parse (data) . ok () ? ; let member = archive . members () . filter_map (Result :: ok) . find (| m | m . name () == member_name) ? ; member . data (data) . ok () ? } None => data , } ; let (macho , data) = find_header (data) ? ; let endian = macho . endian () . ok () ? ; let obj = Object :: parse (macho , endian , data) ? ; Context :: new (stash , obj , None , None) }) }
}

macro_rules! handle_split_dwarf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_split_dwarf in module {}", module_path!());
    };
}

mkfn!{
    handle_split_dwarf_introspect!();
    pub (super) fn handle_split_dwarf < 'data > (_package : Option < & gimli :: DwarfPackage < EndianSlice < 'data , Endian > > > , _stash : & 'data Stash , _load : addr2line :: SplitDwarfLoad < EndianSlice < 'data , Endian > > ,) -> Option < Arc < gimli :: Dwarf < EndianSlice < 'data , Endian > > > > { None }
}