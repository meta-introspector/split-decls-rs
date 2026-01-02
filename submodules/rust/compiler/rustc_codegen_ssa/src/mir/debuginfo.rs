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
mkuse!{use std :: collections :: hash_map :: Entry ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use rustc_abi :: { BackendRepr , FieldIdx , FieldsShape , Size , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { Instance , Ty } ;}
mkuse!{use rustc_middle :: { bug , mir , ty } ;}
mkuse!{use rustc_session :: config :: DebugInfo ;}
mkuse!{use rustc_span :: { BytePos , Span , Symbol , hygiene , sym } ;}
mkuse!{use super :: operand :: { OperandRef , OperandValue } ;}
mkuse!{use super :: place :: { PlaceRef , PlaceValue } ;}
mkuse!{use super :: { FunctionCx , LocalRef , PerLocalVarDebugInfoIndexVec } ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkstruct!{pub struct FunctionDebugContext < 'tcx , S , L > { # [doc = " Maps from source code to the corresponding debug info scope."] pub scopes : IndexVec < mir :: SourceScope , DebugScope < S , L > > , # [doc = " Maps from an inlined function to its debug info declaration."] pub inlined_function_scopes : FxHashMap < Instance < 'tcx > , S > , }}}
mkitem!{mkenum!{# [derive (Copy , Clone)] pub enum VariableKind { ArgumentVariable (usize) , LocalVariable , }}}
mkitem!{mkstruct!{# [doc = " Like `mir::VarDebugInfo`, but within a `mir::Local`."] # [derive (Clone)] pub struct PerLocalVarDebugInfo < 'tcx , D > { pub name : Symbol , pub source_info : mir :: SourceInfo , # [doc = " `DIVariable` returned by `create_dbg_var`."] pub dbg_var : Option < D > , # [doc = " Byte range in the `dbg_var` covered by this fragment,"] # [doc = " if this is a fragment of a composite `VarDebugInfo`."] pub fragment : Option < Range < Size > > , # [doc = " `.place.projection` from `mir::VarDebugInfo`."] pub projection : & 'tcx ty :: List < mir :: PlaceElem < 'tcx > > , }}}
mkitem!{mkstruct!{# [doc = " Information needed to emit a constant."] pub struct ConstDebugInfo < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > { pub name : String , pub source_info : mir :: SourceInfo , pub operand : OperandRef < 'tcx , Bx :: Value > , pub dbg_var : Bx :: DIVariable , pub dbg_loc : Bx :: DILocation , pub fragment : Option < Range < Size > > , pub _phantom : PhantomData < & 'a () > , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , Debug)] pub struct DebugScope < S , L > { pub dbg_scope : S , # [doc = " Call site location, if this scope was inlined from another function."] pub inlined_at : Option < L > , pub file_start_pos : BytePos , pub file_end_pos : BytePos , }}}
mkitem!{mkimpl!{impl < 'tcx , S : Copy , L : Copy > DebugScope < S , L > { # [doc = " DILocations inherit source file name from the parent DIScope. Due to macro expansions"] # [doc = " it may so happen that the current span belongs to a different file than the DIScope"] # [doc = " corresponding to span's containing source scope. If so, we need to create a DIScope"] # [doc = " \"extension\" into that file."] pub fn adjust_dbg_scope_for_span < Cx : CodegenMethods < 'tcx , DIScope = S , DILocation = L > > (& self , cx : & Cx , span : Span ,) -> S { let pos = span . lo () ; if pos < self . file_start_pos || pos >= self . file_end_pos { let sm = cx . sess () . source_map () ; cx . extend_scope_to_file (self . dbg_scope , & sm . lookup_char_pos (pos) . file) } else { self . dbg_scope } } }}}
mkitem!{mktrait!{trait DebugInfoOffsetLocation < 'tcx , Bx > { fn deref (& self , bx : & mut Bx) -> Self ; fn layout (& self) -> TyAndLayout < 'tcx > ; fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self ; fn project_constant_index (& self , bx : & mut Bx , offset : u64) -> Self ; fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self ; }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > DebugInfoOffsetLocation < 'tcx , Bx > for PlaceRef < 'tcx , Bx :: Value > { fn deref (& self , bx : & mut Bx) -> Self { bx . load_operand (* self) . deref (bx . cx ()) } fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self { PlaceRef :: project_field (* self , bx , field . index ()) } fn project_constant_index (& self , bx : & mut Bx , offset : u64) -> Self { let lloffset = bx . cx () . const_usize (offset) ; self . project_index (bx , lloffset) } fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self { self . project_downcast (bx , variant) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > DebugInfoOffsetLocation < 'tcx , Bx > for TyAndLayout < 'tcx > { fn deref (& self , bx : & mut Bx) -> Self { bx . cx () . layout_of (self . ty . builtin_deref (true) . unwrap_or_else (| | bug ! ("cannot deref `{}`" , self . ty)) ,) } fn layout (& self) -> TyAndLayout < 'tcx > { * self } fn project_field (& self , bx : & mut Bx , field : FieldIdx) -> Self { self . field (bx . cx () , field . index ()) } fn project_constant_index (& self , bx : & mut Bx , index : u64) -> Self { self . field (bx . cx () , index as usize) } fn downcast (& self , bx : & mut Bx , variant : VariantIdx) -> Self { self . for_variant (bx . cx () , variant) } }}}
mkitem!{mkstruct!{struct DebugInfoOffset < T > { # [doc = " Offset from the `base` used to calculate the debuginfo offset."] direct_offset : Size , # [doc = " Each offset in this vector indicates one level of indirection from the base or previous"] # [doc = " indirect offset plus a dereference."] indirect_offsets : Vec < Size > , # [doc = " The final location debuginfo should point to."] result : T , }}}

macro_rules! calculate_debuginfo_offset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calculate_debuginfo_offset in module {}", module_path!());
    };
}

mkfn!{
    calculate_debuginfo_offset_introspect!();
    fn calculate_debuginfo_offset < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > , L : DebugInfoOffsetLocation < 'tcx , Bx > , > (bx : & mut Bx , projection : & [mir :: PlaceElem < 'tcx >] , base : L ,) -> DebugInfoOffset < L > { let mut direct_offset = Size :: ZERO ; let mut indirect_offsets = vec ! [] ; let mut place = base ; for elem in projection { match * elem { mir :: ProjectionElem :: Deref => { indirect_offsets . push (Size :: ZERO) ; place = place . deref (bx) ; } mir :: ProjectionElem :: Field (field , _) => { let offset = indirect_offsets . last_mut () . unwrap_or (& mut direct_offset) ; * offset += place . layout () . fields . offset (field . index ()) ; place = place . project_field (bx , field) ; } mir :: ProjectionElem :: Downcast (_ , variant) => { place = place . downcast (bx , variant) ; } mir :: ProjectionElem :: ConstantIndex { offset : index , min_length : _ , from_end : false , } => { let offset = indirect_offsets . last_mut () . unwrap_or (& mut direct_offset) ; let FieldsShape :: Array { stride , count : _ } = place . layout () . fields else { bug ! ("ConstantIndex on non-array type {:?}" , place . layout ()) } ; * offset += stride * index ; place = place . project_constant_index (bx , index) ; } _ => { assert ! (! elem . can_use_in_debuginfo ()) ; bug ! ("unsupported var debuginfo projection `{:?}`" , projection) } } } DebugInfoOffset { direct_offset , indirect_offsets , result : place } }
}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub fn set_debug_loc (& self , bx : & mut Bx , source_info : mir :: SourceInfo) { bx . set_span (source_info . span) ; if let Some (dbg_loc) = self . dbg_loc (source_info) { bx . set_dbg_loc (dbg_loc) ; } } fn dbg_loc (& self , source_info : mir :: SourceInfo) -> Option < Bx :: DILocation > { let (dbg_scope , inlined_at , span) = self . adjusted_span_and_dbg_scope (source_info) ? ; Some (self . cx . dbg_loc (dbg_scope , inlined_at , span)) } fn adjusted_span_and_dbg_scope (& self , source_info : mir :: SourceInfo ,) -> Option < (Bx :: DIScope , Option < Bx :: DILocation > , Span) > { let scope = & self . debug_context . as_ref () ? . scopes [source_info . scope] ; let span = hygiene :: walk_chain_collapsed (source_info . span , self . mir . span) ; Some ((scope . adjust_dbg_scope_for_span (self . cx , span) , scope . inlined_at , span)) } fn spill_operand_to_stack (operand : OperandRef < 'tcx , Bx :: Value > , name : Option < String > , bx : & mut Bx ,) -> PlaceRef < 'tcx , Bx :: Value > { let spill_slot = PlaceRef :: alloca (bx , operand . layout) ; if let Some (name) = name { bx . set_var_name (spill_slot . val . llval , & (name + ".dbg.spill")) ; } operand . val . store (bx , spill_slot) ; spill_slot } # [doc = " Apply debuginfo and/or name, after creating the `alloca` for a local,"] # [doc = " or initializing the local with an operand (whichever applies)."] pub (crate) fn debug_introduce_local (& self , bx : & mut Bx , local : mir :: Local) { let full_debug_info = bx . sess () . opts . debuginfo == DebugInfo :: Full ; let vars = match & self . per_local_var_debug_info { Some (per_local) => & per_local [local] , None => return , } ; let whole_local_var = vars . iter () . find (| var | var . projection . is_empty ()) . cloned () ; let has_proj = | | vars . iter () . any (| var | ! var . projection . is_empty ()) ; let fallback_var = if self . mir . local_kind (local) == mir :: LocalKind :: Arg { let arg_index = local . index () - 1 ; if arg_index == 0 && has_proj () { None } else if whole_local_var . is_some () { None } else { let name = sym :: empty ; let decl = & self . mir . local_decls [local] ; let dbg_var = if full_debug_info { self . adjusted_span_and_dbg_scope (decl . source_info) . map (| (dbg_scope , _ , span) | { let kind = VariableKind :: ArgumentVariable (arg_index + 1) ; let arg_ty = self . monomorphize (decl . ty) ; self . cx . create_dbg_var (name , arg_ty , dbg_scope , kind , span) } ,) } else { None } ; Some (PerLocalVarDebugInfo { name , source_info : decl . source_info , dbg_var , fragment : None , projection : ty :: List :: empty () , }) } } else { None } ; let local_ref = & self . locals [local] ; let name = if bx . sess () . fewer_names () { None } else { Some (match whole_local_var . or_else (| | fallback_var . clone ()) { Some (var) if var . name != sym :: empty => var . name . to_string () , _ => format ! ("{local:?}") , }) } ; if let Some (name) = & name { match local_ref { LocalRef :: Place (place) | LocalRef :: UnsizedPlace (place) => { bx . set_var_name (place . val . llval , name) ; } LocalRef :: Operand (operand) => match operand . val { OperandValue :: Ref (PlaceValue { llval : x , .. }) | OperandValue :: Immediate (x) => { bx . set_var_name (x , name) ; } OperandValue :: Pair (a , b) => { bx . set_var_name (a , & (name . clone () + ".0")) ; bx . set_var_name (b , & (name . clone () + ".1")) ; } OperandValue :: ZeroSized => { } } , LocalRef :: PendingOperand => { } } } if ! full_debug_info || vars . is_empty () && fallback_var . is_none () { return ; } let base = match local_ref { LocalRef :: PendingOperand => return , LocalRef :: Operand (operand) => { let attrs = bx . tcx () . codegen_instance_attrs (self . instance . def) ; if attrs . flags . contains (CodegenFnAttrFlags :: NAKED) { return ; } Self :: spill_operand_to_stack (* operand , name , bx) } LocalRef :: Place (place) => * place , LocalRef :: UnsizedPlace (_) => return , } ; let vars = vars . iter () . cloned () . chain (fallback_var) ; for var in vars { self . debug_introduce_local_as_var (bx , local , base , var) ; } } fn debug_introduce_local_as_var (& self , bx : & mut Bx , local : mir :: Local , base : PlaceRef < 'tcx , Bx :: Value > , var : PerLocalVarDebugInfo < 'tcx , Bx :: DIVariable > ,) { let Some (dbg_var) = var . dbg_var else { return } ; let Some (dbg_loc) = self . dbg_loc (var . source_info) else { return } ; let DebugInfoOffset { direct_offset , indirect_offsets , result : _ } = calculate_debuginfo_offset (bx , var . projection , base . layout) ; let should_create_individual_allocas = bx . cx () . sess () . target . is_like_msvc && self . mir . local_kind (local) == mir :: LocalKind :: Arg && (direct_offset != Size :: ZERO || ! matches ! (& indirect_offsets [..] , [Size :: ZERO] | [])) ; if should_create_individual_allocas { let DebugInfoOffset { direct_offset : _ , indirect_offsets : _ , result : place } = calculate_debuginfo_offset (bx , var . projection , base) ; let ptr_ty = Ty :: new_mut_ptr (bx . tcx () , place . layout . ty) ; let ptr_layout = bx . layout_of (ptr_ty) ; let alloca = PlaceRef :: alloca (bx , ptr_layout) ; bx . set_var_name (alloca . val . llval , & (var . name . to_string () + ".dbg.spill")) ; bx . store_to_place (place . val . llval , alloca . val) ; bx . dbg_var_addr (dbg_var , dbg_loc , alloca . val . llval , Size :: ZERO , & [Size :: ZERO] , var . fragment ,) ; } else { bx . dbg_var_addr (dbg_var , dbg_loc , base . val . llval , direct_offset , & indirect_offsets , var . fragment ,) ; } } pub (crate) fn debug_introduce_locals (& self , bx : & mut Bx , consts : Vec < ConstDebugInfo < 'a , 'tcx , Bx > > ,) { if bx . sess () . opts . debuginfo == DebugInfo :: Full || ! bx . sess () . fewer_names () { for local in self . locals . indices () { self . debug_introduce_local (bx , local) ; } for ConstDebugInfo { name , source_info , operand , dbg_var , dbg_loc , fragment , .. } in consts . into_iter () { self . set_debug_loc (bx , source_info) ; let base = FunctionCx :: spill_operand_to_stack (operand , Some (name) , bx) ; bx . clear_dbg_loc () ; bx . dbg_var_addr (dbg_var , dbg_loc , base . val . llval , Size :: ZERO , & [] , fragment) ; } } } # [doc = " Partition all `VarDebugInfo` in `self.mir`, by their base `Local`."] pub (crate) fn compute_per_local_var_debug_info (& self , bx : & mut Bx ,) -> Option < (PerLocalVarDebugInfoIndexVec < 'tcx , Bx :: DIVariable > , Vec < ConstDebugInfo < 'a , 'tcx , Bx > > ,) > { let full_debug_info = self . cx . sess () . opts . debuginfo == DebugInfo :: Full ; let target_is_msvc = self . cx . sess () . target . is_like_msvc ; if ! full_debug_info && self . cx . sess () . fewer_names () { return None ; } let mut per_local = IndexVec :: from_elem (vec ! [] , & self . mir . local_decls) ; let mut constants = vec ! [] ; let mut params_seen : FxHashMap < _ , Bx :: DIVariable > = Default :: default () ; for var in & self . mir . var_debug_info { let dbg_scope_and_span = if full_debug_info { self . adjusted_span_and_dbg_scope (var . source_info) } else { None } ; let var_ty = if let Some (ref fragment) = var . composite { self . monomorphize (fragment . ty) } else { match var . value { mir :: VarDebugInfoContents :: Place (place) => { self . monomorphized_place_ty (place . as_ref ()) } mir :: VarDebugInfoContents :: Const (c) => self . monomorphize (c . ty ()) , } } ; let dbg_var = dbg_scope_and_span . map (| (dbg_scope , _ , span) | { let var_kind = if let Some (arg_index) = var . argument_index && var . composite . is_none () && let mir :: VarDebugInfoContents :: Place (place) = var . value && place . projection . is_empty () { let arg_index = arg_index as usize ; if target_is_msvc { let var_ty_layout = self . cx . layout_of (var_ty) ; if let BackendRepr :: ScalarPair (_ , _) = var_ty_layout . backend_repr { VariableKind :: LocalVariable } else { VariableKind :: ArgumentVariable (arg_index) } } else { VariableKind :: ArgumentVariable (arg_index) } } else { VariableKind :: LocalVariable } ; if let VariableKind :: ArgumentVariable (arg_index) = var_kind { match params_seen . entry ((dbg_scope , arg_index)) { Entry :: Occupied (o) => o . get () . clone () , Entry :: Vacant (v) => v . insert (self . cx . create_dbg_var (var . name , var_ty , dbg_scope , var_kind , span) ,) . clone () , } } else { self . cx . create_dbg_var (var . name , var_ty , dbg_scope , var_kind , span) } }) ; let fragment = if let Some (ref fragment) = var . composite { let var_layout = self . cx . layout_of (var_ty) ; let DebugInfoOffset { direct_offset , indirect_offsets , result : fragment_layout } = calculate_debuginfo_offset (bx , & fragment . projection , var_layout) ; assert ! (indirect_offsets . is_empty ()) ; if fragment_layout . size == Size :: ZERO { continue ; } else if fragment_layout . size == var_layout . size { None } else { Some (direct_offset .. direct_offset + fragment_layout . size) } } else { None } ; match var . value { mir :: VarDebugInfoContents :: Place (place) => { per_local [place . local] . push (PerLocalVarDebugInfo { name : var . name , source_info : var . source_info , dbg_var , fragment , projection : place . projection , }) ; } mir :: VarDebugInfoContents :: Const (c) => { if let Some (dbg_var) = dbg_var { let Some (dbg_loc) = self . dbg_loc (var . source_info) else { continue } ; let operand = self . eval_mir_constant_to_operand (bx , & c) ; constants . push (ConstDebugInfo { name : var . name . to_string () , source_info : var . source_info , operand , dbg_var , dbg_loc , fragment , _phantom : PhantomData , }) ; } } } } Some ((per_local , constants)) } }}}