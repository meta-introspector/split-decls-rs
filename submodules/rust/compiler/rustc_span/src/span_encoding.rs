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
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_serialize :: int_overflow :: DebugStrictAdd ;}
mkuse!{use crate :: def_id :: { DefIndex , LocalDefId } ;}
mkuse!{use crate :: hygiene :: SyntaxContext ;}
mkuse!{use crate :: { BytePos , SPAN_TRACK , SpanData } ;}
mkitem!{mkstruct!{#[doc = " A compressed span."] #[doc = ""] #[doc = " [`SpanData`] is 16 bytes, which is too big to stick everywhere. `Span` only"] #[doc = " takes up 8 bytes, with less space for the length, parent and context. The"] #[doc = " vast majority (99.9%+) of `SpanData` instances can be made to fit within"] #[doc = " those 8 bytes. Any `SpanData` whose fields don't fit into a `Span` are"] #[doc = " stored in a separate interner table, and the `Span` will index into that"] #[doc = " table. Interning is rare enough that the cost is low, but common enough"] #[doc = " that the code is exercised regularly."] #[doc = ""] #[doc = " An earlier version of this code used only 4 bytes for `Span`, but that was"] #[doc = " slower because only 80--90% of spans could be stored inline (even less in"] #[doc = " very large crates) and so the interner was used a lot more. That version of"] #[doc = " the code also predated the storage of parents."] #[doc = ""] #[doc = " There are four different span forms."] #[doc = ""] #[doc = " Inline-context format (requires non-huge length, non-huge context, and no parent):"] #[doc = " - `span.lo_or_index == span_data.lo`"] #[doc = " - `span.len_with_tag_or_marker == len == span_data.hi - span_data.lo` (must be `<= MAX_LEN`)"] #[doc = " - `span.ctxt_or_parent_or_marker == span_data.ctxt` (must be `<= MAX_CTXT`)"] #[doc = ""] #[doc = " Inline-parent format (requires non-huge length, root context, and non-huge parent):"] #[doc = " - `span.lo_or_index == span_data.lo`"] #[doc = " - `span.len_with_tag_or_marker & !PARENT_TAG == len == span_data.hi - span_data.lo`"] #[doc = "   (must be `<= MAX_LEN`)"] #[doc = " - `span.len_with_tag_or_marker` has top bit (`PARENT_TAG`) set"] #[doc = " - `span.ctxt_or_parent_or_marker == span_data.parent` (must be `<= MAX_CTXT`)"] #[doc = ""] #[doc = " Partially-interned format (requires non-huge context):"] #[doc = " - `span.lo_or_index == index` (indexes into the interner table)"] #[doc = " - `span.len_with_tag_or_marker == BASE_LEN_INTERNED_MARKER`"] #[doc = " - `span.ctxt_or_parent_or_marker == span_data.ctxt` (must be `<= MAX_CTXT`)"] #[doc = ""] #[doc = " Fully-interned format (all cases not covered above):"] #[doc = " - `span.lo_or_index == index` (indexes into the interner table)"] #[doc = " - `span.len_with_tag_or_marker == BASE_LEN_INTERNED_MARKER`"] #[doc = " - `span.ctxt_or_parent_or_marker == CTXT_INTERNED_MARKER`"] #[doc = ""] #[doc = " The partially-interned form requires looking in the interning table for"] #[doc = " lo and length, but the context is stored inline as well as interned."] #[doc = " This is useful because context lookups are often done in isolation, and"] #[doc = " inline lookups are quicker."] #[doc = ""] #[doc = " Notes about the choice of field sizes:"] #[doc = " - `lo` is 32 bits in both `Span` and `SpanData`, which means that `lo`"] #[doc = "   values never cause interning. The number of bits needed for `lo`"] #[doc = "   depends on the crate size. 32 bits allows up to 4 GiB of code in a crate."] #[doc = "   Having no compression on this field means there is no performance cliff"] #[doc = "   if a crate exceeds a particular size."] #[doc = " - `len` is ~15 bits in `Span` (a u16, minus 1 bit for PARENT_TAG) and 32"] #[doc = "   bits in `SpanData`, which means that large `len` values will cause"] #[doc = "   interning. The number of bits needed for `len` does not depend on the"] #[doc = "   crate size. The most common numbers of bits for `len` are from 0 to 7,"] #[doc = "   with a peak usually at 3 or 4, and then it drops off quickly from 8"] #[doc = "   onwards. 15 bits is enough for 99.99%+ of cases, but larger values"] #[doc = "   (sometimes 20+ bits) might occur dozens of times in a typical crate."] #[doc = " - `ctxt_or_parent_or_marker` is 16 bits in `Span` and two 32 bit fields in"] #[doc = "   `SpanData`, which means intering will happen if `ctxt` is large, if"] #[doc = "   `parent` is large, or if both values are non-zero. The number of bits"] #[doc = "   needed for `ctxt` values depend partly on the crate size and partly on"] #[doc = "   the form of the code. No crates in `rustc-perf` need more than 15 bits"] #[doc = "   for `ctxt_or_parent_or_marker`, but larger crates might need more than 16"] #[doc = "   bits. The number of bits needed for `parent` hasn't been measured,"] #[doc = "   because `parent` isn't currently used by default."] #[doc = ""] #[doc = " In order to reliably use parented spans in incremental compilation,"] #[doc = " accesses to `lo` and `hi` must introduce a dependency to the parent definition's span."] #[doc = " This is performed using the callback `SPAN_TRACK` to access the query engine."] #[derive (Clone , Copy , Eq , PartialEq , Hash)] #[rustc_pass_by_value] pub struct Span { lo_or_index : u32 , len_with_tag_or_marker : u16 , ctxt_or_parent_or_marker : u16 , }}}
mkitem!{mkstruct!{#[derive (Clone , Copy)] struct InlineCtxt { lo : u32 , len : u16 , ctxt : u16 , }}}
mkitem!{mkstruct!{#[derive (Clone , Copy)] struct InlineParent { lo : u32 , len_with_tag : u16 , parent : u16 , }}}
mkitem!{mkstruct!{#[derive (Clone , Copy)] struct PartiallyInterned { index : u32 , ctxt : u16 , }}}
mkitem!{mkstruct!{#[derive (Clone , Copy)] struct Interned { index : u32 , }}}
mkitem!{mkimpl!{impl InlineCtxt { #[inline] fn data (self) -> SpanData { let len = self . len as u32 ; debug_assert ! (len <= MAX_LEN) ; SpanData { lo : BytePos (self . lo) , hi : BytePos (self . lo . debug_strict_add (len)) , ctxt : SyntaxContext :: from_u16 (self . ctxt) , parent : None , } } #[inline] fn span (lo : u32 , len : u16 , ctxt : u16) -> Span { Span { lo_or_index : lo , len_with_tag_or_marker : len , ctxt_or_parent_or_marker : ctxt } } #[inline] fn from_span (span : Span) -> InlineCtxt { let (lo , len , ctxt) = (span . lo_or_index , span . len_with_tag_or_marker , span . ctxt_or_parent_or_marker) ; InlineCtxt { lo , len , ctxt } } }}}
mkitem!{mkimpl!{impl InlineParent { #[inline] fn data (self) -> SpanData { let len = (self . len_with_tag & ! PARENT_TAG) as u32 ; debug_assert ! (len <= MAX_LEN) ; SpanData { lo : BytePos (self . lo) , hi : BytePos (self . lo . debug_strict_add (len)) , ctxt : SyntaxContext :: root () , parent : Some (LocalDefId { local_def_index : DefIndex :: from_u16 (self . parent) }) , } } #[inline] fn span (lo : u32 , len : u16 , parent : u16) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (lo , PARENT_TAG | len , parent) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } #[inline] fn from_span (span : Span) -> InlineParent { let (lo , len_with_tag , parent) = (span . lo_or_index , span . len_with_tag_or_marker , span . ctxt_or_parent_or_marker) ; InlineParent { lo , len_with_tag , parent } } }}}
mkitem!{mkimpl!{impl PartiallyInterned { #[inline] fn data (self) -> SpanData { SpanData { ctxt : SyntaxContext :: from_u16 (self . ctxt) , .. with_span_interner (| interner | interner . spans [self . index as usize]) } } #[inline] fn span (index : u32 , ctxt : u16) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (index , BASE_LEN_INTERNED_MARKER , ctxt) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } #[inline] fn from_span (span : Span) -> PartiallyInterned { PartiallyInterned { index : span . lo_or_index , ctxt : span . ctxt_or_parent_or_marker } } }}}
mkitem!{mkimpl!{impl Interned { #[inline] fn data (self) -> SpanData { with_span_interner (| interner | interner . spans [self . index as usize]) } #[inline] fn span (index : u32) -> Span { let (lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker) = (index , BASE_LEN_INTERNED_MARKER , CTXT_INTERNED_MARKER) ; Span { lo_or_index , len_with_tag_or_marker , ctxt_or_parent_or_marker } } #[inline] fn from_span (span : Span) -> Interned { Interned { index : span . lo_or_index } } }}}
mkitem!{macro_rules ! match_span_kind { ($ span : expr , InlineCtxt ($ span1 : ident) => $ arm1 : expr , InlineParent ($ span2 : ident) => $ arm2 : expr , PartiallyInterned ($ span3 : ident) => $ arm3 : expr , Interned ($ span4 : ident) => $ arm4 : expr ,) => { if $ span . len_with_tag_or_marker != BASE_LEN_INTERNED_MARKER { if $ span . len_with_tag_or_marker & PARENT_TAG == 0 { let $ span1 = InlineCtxt :: from_span ($ span) ; $ arm1 } else { let $ span2 = InlineParent :: from_span ($ span) ; $ arm2 } } else if $ span . ctxt_or_parent_or_marker != CTXT_INTERNED_MARKER { let $ span3 = PartiallyInterned :: from_span ($ span) ; $ arm3 } else { let $ span4 = Interned :: from_span ($ span) ; $ arm4 } } ; }}
mkitem!{const MAX_LEN : u32 = 0b0111_1111_1111_1110 ;}
mkitem!{const MAX_CTXT : u32 = 0b0111_1111_1111_1110 ;}
mkitem!{const PARENT_TAG : u16 = 0b1000_0000_0000_0000 ;}
mkitem!{const BASE_LEN_INTERNED_MARKER : u16 = 0b1111_1111_1111_1111 ;}
mkitem!{const CTXT_INTERNED_MARKER : u16 = 0b1111_1111_1111_1111 ;}
mkitem!{#[doc = " The dummy span has zero position, length, and context, and no parent."] pub const DUMMY_SP : Span = Span { lo_or_index : 0 , len_with_tag_or_marker : 0 , ctxt_or_parent_or_marker : 0 } ;}
mkitem!{mkimpl!{impl Span { #[inline] pub fn new (mut lo : BytePos , mut hi : BytePos , ctxt : SyntaxContext , parent : Option < LocalDefId > ,) -> Self { if lo > hi { std :: mem :: swap (& mut lo , & mut hi) ; } let (len , ctxt32) = (hi . 0 - lo . 0 , ctxt . as_u32 ()) ; if len <= MAX_LEN && ctxt32 <= MAX_CTXT { match parent { None => return InlineCtxt :: span (lo . 0 , len as u16 , ctxt32 as u16) , Some (parent) => { let parent32 = parent . local_def_index . as_u32 () ; if ctxt32 == 0 && parent32 <= MAX_CTXT { return InlineParent :: span (lo . 0 , len as u16 , parent32 as u16) ; } } } } let index = | ctxt | { with_span_interner (| interner | interner . intern (& SpanData { lo , hi , ctxt , parent })) } ; if ctxt32 <= MAX_CTXT { PartiallyInterned :: span (index (SyntaxContext :: from_u32 (u32 :: MAX)) , ctxt32 as u16) } else { Interned :: span (index (ctxt)) } } #[inline] pub fn data (self) -> SpanData { let data = self . data_untracked () ; if let Some (parent) = data . parent { (* SPAN_TRACK) (parent) ; } data } #[doc = " Internal function to translate between an encoded span and the expanded representation."] #[doc = " This function must not be used outside the incremental engine."] #[inline] pub fn data_untracked (self) -> SpanData { match_span_kind ! { self , InlineCtxt (span) => span . data () , InlineParent (span) => span . data () , PartiallyInterned (span) => span . data () , Interned (span) => span . data () , } } #[doc = " Returns `true` if this span comes from any kind of macro, desugaring or inlining."] #[inline] pub fn from_expansion (self) -> bool { let ctxt = match_span_kind ! { self , InlineCtxt (span) => SyntaxContext :: from_u16 (span . ctxt) , InlineParent (_span) => SyntaxContext :: root () , PartiallyInterned (span) => SyntaxContext :: from_u16 (span . ctxt) , Interned (_span) => SyntaxContext :: from_u16 (CTXT_INTERNED_MARKER) , } ; ! ctxt . is_root () } #[doc = " Returns `true` if this is a dummy span with any hygienic context."] #[inline] pub fn is_dummy (self) -> bool { if self . len_with_tag_or_marker != BASE_LEN_INTERNED_MARKER { let lo = self . lo_or_index ; let len = (self . len_with_tag_or_marker & ! PARENT_TAG) as u32 ; debug_assert ! (len <= MAX_LEN) ; lo == 0 && len == 0 } else { let index = self . lo_or_index ; let data = with_span_interner (| interner | interner . spans [index as usize]) ; data . lo == BytePos (0) && data . hi == BytePos (0) } } #[inline] pub fn map_ctxt (self , map : impl FnOnce (SyntaxContext) -> SyntaxContext) -> Span { let data = match_span_kind ! { self , InlineCtxt (span) => { let new_ctxt = map (SyntaxContext :: from_u16 (span . ctxt)) ; let new_ctxt32 = new_ctxt . as_u32 () ; return if new_ctxt32 <= MAX_CTXT { InlineCtxt :: span (span . lo , span . len , new_ctxt32 as u16) } else { span . data () . with_ctxt (new_ctxt) } ; } , InlineParent (span) => span . data () , PartiallyInterned (span) => span . data () , Interned (span) => span . data () , } ; data . with_ctxt (map (data . ctxt)) } #[inline] fn inline_ctxt (self) -> Result < SyntaxContext , usize > { match_span_kind ! { self , InlineCtxt (span) => Ok (SyntaxContext :: from_u16 (span . ctxt)) , InlineParent (_span) => Ok (SyntaxContext :: root ()) , PartiallyInterned (span) => Ok (SyntaxContext :: from_u16 (span . ctxt)) , Interned (span) => Err (span . index as usize) , } } #[doc = " This function is used as a fast path when decoding the full `SpanData` is not necessary."] #[doc = " It's a cut-down version of `data_untracked`."] #[cfg_attr (not (test) , rustc_diagnostic_item = "SpanCtxt")] #[inline] pub fn ctxt (self) -> SyntaxContext { self . inline_ctxt () . unwrap_or_else (| index | with_span_interner (| interner | interner . spans [index] . ctxt)) } #[inline] pub fn eq_ctxt (self , other : Span) -> bool { match (self . inline_ctxt () , other . inline_ctxt ()) { (Ok (ctxt1) , Ok (ctxt2)) => ctxt1 == ctxt2 , (Ok (_) , Err (_)) | (Err (_) , Ok (_)) => false , (Err (index1) , Err (index2)) => with_span_interner (| interner | { interner . spans [index1] . ctxt == interner . spans [index2] . ctxt }) , } } #[inline] pub fn with_parent (self , parent : Option < LocalDefId >) -> Span { let data = match_span_kind ! { self , InlineCtxt (span) => { match parent { None => return self , Some (parent) => { let parent32 = parent . local_def_index . as_u32 () ; if span . ctxt == 0 && parent32 <= MAX_CTXT { return InlineParent :: span (span . lo , span . len , parent32 as u16) ; } } } span . data () } , InlineParent (span) => span . data () , PartiallyInterned (span) => span . data () , Interned (span) => span . data () , } ; if let Some (old_parent) = data . parent { (* SPAN_TRACK) (old_parent) ; } data . with_parent (parent) } #[inline] pub fn parent (self) -> Option < LocalDefId > { let interned_parent = | index : u32 | with_span_interner (| interner | interner . spans [index as usize] . parent) ; match_span_kind ! { self , InlineCtxt (_span) => None , InlineParent (span) => Some (LocalDefId { local_def_index : DefIndex :: from_u16 (span . parent) }) , PartiallyInterned (span) => interned_parent (span . index) , Interned (span) => interned_parent (span . index) , } } }}}
mkitem!{mkstruct!{#[derive (Default)] pub (crate) struct SpanInterner { spans : FxIndexSet < SpanData > , }}}
mkitem!{mkimpl!{impl SpanInterner { fn intern (& mut self , span_data : & SpanData) -> u32 { let (index , _) = self . spans . insert_full (* span_data) ; index as u32 } }}}

macro_rules! with_span_interner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_span_interner in module {}", module_path!());
    };
}

mkfn!{
    with_span_interner_introspect!();
    #[inline] fn with_span_interner < T , F : FnOnce (& mut SpanInterner) -> T > (f : F) -> T { crate :: with_session_globals (| session_globals | f (& mut session_globals . span_interner . lock ())) }
}