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
mkuse!{use core :: fmt :: { self , Write } ;}
mkuse!{use core :: slice :: from_raw_parts ;}
mkuse!{use libc :: c_char ;}
mkuse!{use object :: NativeEndian as NE ;}
mkitem!{unsafe extern "C" { #[allow (improper_ctypes)] fn dl_iterate_phdr (f : extern "C" fn (info : & dl_phdr_info , size : usize , data : & mut DsoPrinter < '_ , '_ >) -> i32 , data : & mut DsoPrinter < '_ , '_ > ,) -> i32 ; }}
mkitem!{const PT_LOAD : u32 = 1 ;}
mkitem!{const PT_NOTE : u32 = 4 ;}
mkitem!{mkstruct!{#[allow (non_camel_case_types)] #[repr (C)] struct dl_phdr_info { addr : * const u8 , name : * const c_char , phdr : * const Elf_Phdr , phnum : u16 , adds : u64 , subs : u64 , tls_modid : usize , tls_data : * const u8 , }}}
mkitem!{mkimpl!{impl dl_phdr_info { fn program_headers (& self) -> PhdrIter < '_ > { PhdrIter { phdrs : self . phdr_slice () , base : self . addr , } } fn phdr_slice (& self) -> & [Elf_Phdr] { unsafe { from_raw_parts (self . phdr , self . phnum as usize) } } }}}
mkitem!{mkstruct!{struct PhdrIter < 'a > { phdrs : & 'a [Elf_Phdr] , base : * const u8 , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for PhdrIter < 'a > { type Item = Phdr < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . phdrs . split_first () . map (| (phdr , new_phdrs) | { self . phdrs = new_phdrs ; Phdr { phdr , base : self . base , } }) } }}}
mkitem!{mkstruct!{#[allow (non_camel_case_types)] #[derive (Clone , Debug)] #[repr (C)] struct Elf_Phdr { p_type : u32 , p_flags : u32 , p_offset : u64 , p_vaddr : u64 , p_paddr : u64 , p_filesz : u64 , p_memsz : u64 , p_align : u64 , }}}
mkitem!{mkstruct!{struct Phdr < 'a > { phdr : & 'a Elf_Phdr , base : * const u8 , }}}
mkitem!{mkimpl!{impl < 'a > Phdr < 'a > { fn notes (& self) -> NoteIter < 'a > { unsafe { NoteIter :: new (self . base . add (self . phdr . p_offset as usize) , self . phdr . p_memsz as usize ,) } } }}}
mkitem!{const NT_GNU_BUILD_ID : u32 = 3 ;}
mkitem!{#[allow (non_camel_case_types)] type Elf_Nhdr = object :: elf :: NoteHeader32 < NE > ;}
mkitem!{mkstruct!{struct Note < 'a > { name : & 'a [u8] , desc : & 'a [u8] , tipe : u32 , }}}
mkitem!{mkstruct!{struct NoteIter < 'a > { base : & 'a [u8] , error : bool , }}}
mkitem!{mkimpl!{impl < 'a > NoteIter < 'a > { unsafe fn new (base : * const u8 , size : usize) -> Self { NoteIter { base : unsafe { from_raw_parts (base , size) } , error : false , } } }}}

macro_rules! align_to_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_to in module {}", module_path!());
    };
}

mkfn!{
    align_to_introspect!();
    fn align_to (x : usize , to : usize) -> usize { (x + to - 1) & (! to + 1) }
}

macro_rules! take_bytes_align4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function take_bytes_align4 in module {}", module_path!());
    };
}

mkfn!{
    take_bytes_align4_introspect!();
    fn take_bytes_align4 < 'a > (num : usize , bytes : & mut & 'a [u8]) -> Option < & 'a [u8] > { if bytes . len () < align_to (num , 4) { return None ; } let (out , bytes_new) = bytes . split_at (num) ; * bytes = & bytes_new [align_to (num , 4) - num ..] ; Some (out) }
}

macro_rules! take_nhdr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function take_nhdr in module {}", module_path!());
    };
}

mkfn!{
    take_nhdr_introspect!();
    #[doc = " This function has no invariants the caller must uphold, but"] #[doc = " it will return `None`, without mutating, if `bytes` has insufficient size or alignment."] #[doc = " If this returns `Some(nhdr)`, then `bytes` was and remains 4-byte-aligned."] #[doc = " The values in the Elf_Nhdr fields might be nonsense."] fn take_nhdr < 'a > (bytes : & mut & 'a [u8]) -> Option < & 'a Elf_Nhdr > { let (out , rest) = object :: pod :: from_bytes :: < Elf_Nhdr > (bytes) . ok () ? ; * bytes = rest ; Some (out) }
}
mkitem!{mkimpl!{impl < 'a > Iterator for NoteIter < 'a > { type Item = Note < 'a > ; fn next (& mut self) -> Option < Self :: Item > { if self . base . is_empty () || self . error { return None ; } let nhdr = take_nhdr (& mut self . base) ? ; let name = take_bytes_align4 (nhdr . n_namesz . get (NE) as usize , & mut self . base) ? ; let desc = take_bytes_align4 (nhdr . n_descsz . get (NE) as usize , & mut self . base) ? ; Some (Note { name : name , desc : desc , tipe : nhdr . n_type . get (NE) , }) } }}}
mkitem!{mkstruct!{struct Perm (u32) ;}}
mkitem!{#[doc = " Indicates that a segment is executable."] const PERM_X : u32 = 0b00000001 ;}
mkitem!{#[doc = " Indicates that a segment is writable."] const PERM_W : u32 = 0b00000010 ;}
mkitem!{#[doc = " Indicates that a segment is readable."] const PERM_R : u32 = 0b00000100 ;}
mkitem!{mkimpl!{impl core :: fmt :: Display for Perm { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let v = self . 0 ; if v & PERM_R != 0 { f . write_char ('r') ? } if v & PERM_W != 0 { f . write_char ('w') ? } if v & PERM_X != 0 { f . write_char ('x') ? } Ok (()) } }}}
mkitem!{mkstruct!{#[doc = " Represents an ELF segment at runtime."] struct Segment { #[doc = " Gives the runtime virtual address of this segment's contents."] addr : usize , #[doc = " Gives the memory size of this segment's contents."] size : usize , #[doc = " Gives the module virtual address of this segment with the ELF file."] mod_rel_addr : usize , #[doc = " Gives the permissions found in the ELF file. These permissions are not"] #[doc = " necessarily the permissions present at runtime however."] flags : Perm , }}}
mkitem!{mkstruct!{#[doc = " Lets one iterate over Segments from a DSO."] struct SegmentIter < 'a > { phdrs : & 'a [Elf_Phdr] , base : usize , }}}
mkitem!{mkimpl!{impl Iterator for SegmentIter < '_ > { type Item = Segment ; fn next (& mut self) -> Option < Self :: Item > { self . phdrs . split_first () . and_then (| (phdr , new_phdrs) | { self . phdrs = new_phdrs ; if phdr . p_type != PT_LOAD { self . next () } else { Some (Segment { addr : phdr . p_vaddr as usize + self . base , size : phdr . p_memsz as usize , mod_rel_addr : phdr . p_vaddr as usize , flags : Perm (phdr . p_flags) , }) } }) } }}}
mkitem!{mkstruct!{#[doc = " Represents an ELF DSO (Dynamic Shared Object). This type references"] #[doc = " the data stored in the actual DSO rather than making its own copy."] struct Dso < 'a > { #[doc = " The dynamic linker always gives us a name, even if the name is empty."] #[doc = " In the case of the main executable this name will be empty. In the case"] #[doc = " of a shared object it will be the soname (see DT_SONAME)."] name : & 'a str , #[doc = " On Fuchsia virtually all binaries have build IDs but this is not a strict"] #[doc = " requirement. There's no way to match up DSO information with a real ELF"] #[doc = " file afterwards if there is no build_id so we require that every DSO"] #[doc = " have one here. DSO's without a build_id are ignored."] build_id : & 'a [u8] , base : usize , phdrs : & 'a [Elf_Phdr] , }}}
mkitem!{mkimpl!{impl Dso < '_ > { #[doc = " Returns an iterator over Segments in this DSO."] fn segments (& self) -> SegmentIter < '_ > { SegmentIter { phdrs : self . phdrs . as_ref () , base : self . base , } } }}}
mkitem!{mkstruct!{struct HexSlice < 'a > { bytes : & 'a [u8] , }}}
mkitem!{mkimpl!{impl fmt :: Display for HexSlice < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . bytes { write ! (f , "{byte:02x}") ? ; } Ok (()) } }}}

macro_rules! get_build_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_build_id in module {}", module_path!());
    };
}

mkfn!{
    get_build_id_introspect!();
    fn get_build_id < 'a > (info : & 'a dl_phdr_info) -> Option < & 'a [u8] > { for phdr in info . program_headers () { if phdr . phdr . p_type == PT_NOTE { for note in phdr . notes () { if note . tipe == NT_GNU_BUILD_ID && (note . name == b"GNU\0" || note . name == b"GNU") { return Some (note . desc) ; } } } } None }
}
mkitem!{mkenum!{#[doc = " These errors encode issues that arise while parsing information about"] #[doc = " each DSO."] enum Error { #[doc = " NameError means that an error occurred while converting a C style string"] #[doc = " into a rust string."] NameError , #[doc = " BuildIDError means that we didn't find a build ID. This could either be"] #[doc = " because the DSO had no build ID or because the segment containing the"] #[doc = " build ID was malformed."] BuildIDError , }}}

macro_rules! for_each_dso_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function for_each_dso in module {}", module_path!());
    };
}

mkfn!{
    for_each_dso_introspect!();
    #[doc = " Calls either 'dso' or 'error' for each DSO linked into the process by the"] #[doc = " dynamic linker."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc = " * `visitor` - A DsoPrinter that will have one of eats methods called foreach DSO."] fn for_each_dso (mut visitor : & mut DsoPrinter < '_ , '_ >) { extern "C" fn callback (info : & dl_phdr_info , _size : usize , visitor : & mut DsoPrinter < '_ , '_ > ,) -> i32 { let name_len = unsafe { libc :: strlen (info . name) } ; let name_slice : & [u8] = unsafe { core :: slice :: from_raw_parts (info . name . cast :: < u8 > () , name_len) } ; let name = match core :: str :: from_utf8 (name_slice) { Ok (name) => name , Err (_) => { return visitor . error (Error :: NameError) as i32 ; } } ; let build_id = match get_build_id (info) { Some (build_id) => build_id , None => { return visitor . error (Error :: BuildIDError) as i32 ; } } ; visitor . dso (Dso { name : name , build_id : build_id , phdrs : info . phdr_slice () , base : info . addr as usize , }) as i32 } unsafe { dl_iterate_phdr (callback , & mut visitor) } ; }
}
mkitem!{mkstruct!{struct DsoPrinter < 'a , 'b > { writer : & 'a mut core :: fmt :: Formatter < 'b > , module_count : usize , error : core :: fmt :: Result , }}}
mkitem!{mkimpl!{impl DsoPrinter < '_ , '_ > { fn dso (& mut self , dso : Dso < '_ >) -> bool { let mut write = | | { write ! (self . writer , "{{{{{{module:{:#x}:{}:elf:{}}}}}}}\n" , self . module_count , dso . name , HexSlice { bytes : dso . build_id . as_ref () }) ? ; for seg in dso . segments () { write ! (self . writer , "{{{{{{mmap:{:#x}:{:#x}:load:{:#x}:{}:{:#x}}}}}}}\n" , seg . addr , seg . size , self . module_count , seg . flags , seg . mod_rel_addr) ? ; } self . module_count += 1 ; Ok (()) } ; match write () { Ok (()) => false , Err (err) => { self . error = Err (err) ; true } } } fn error (& mut self , _error : Error) -> bool { false } }}}

macro_rules! print_dso_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_dso_context in module {}", module_path!());
    };
}

mkfn!{
    print_dso_context_introspect!();
    #[doc = " This function prints the Fuchsia symbolizer markup for all information contained in a DSO."] pub fn print_dso_context (out : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { out . write_str ("{{{reset:begin}}}\n") ? ; let mut visitor = DsoPrinter { writer : out , module_count : 0 , error : Ok (()) , } ; for_each_dso (& mut visitor) ; visitor . error }
}

macro_rules! finish_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function finish_context in module {}", module_path!());
    };
}

mkfn!{
    finish_context_introspect!();
    #[doc = " This function prints the Fuchsia symbolizer markup to end the backtrace."] pub fn finish_context (out : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { out . write_str ("{{{reset:end}}}\n") }
}