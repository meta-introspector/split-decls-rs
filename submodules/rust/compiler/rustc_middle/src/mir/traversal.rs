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
mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [doc = " Preorder traversal of a graph."] # [doc = ""] # [doc = " Preorder traversal is when each node is visited after at least one of its predecessors. If you"] # [doc = " are familiar with some basic graph theory, then this performs a depth first search and returns"] # [doc = " nodes in order of discovery time."] # [doc = ""] # [doc = " ```text"] # [doc = ""] # [doc = "         A"] # [doc = "        / \\"] # [doc = "       /   \\"] # [doc = "      B     C"] # [doc = "       \\   /"] # [doc = "        \\ /"] # [doc = "         D"] # [doc = " ```"] # [doc = ""] # [doc = " A preorder traversal of this graph is either `A B D C` or `A C D B`"] # [derive (Clone)] pub struct Preorder < 'a , 'tcx > { body : & 'a Body < 'tcx > , visited : DenseBitSet < BasicBlock > , worklist : Vec < BasicBlock > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Preorder < 'a , 'tcx > { pub fn new (body : & 'a Body < 'tcx > , root : BasicBlock) -> Preorder < 'a , 'tcx > { let worklist = vec ! [root] ; Preorder { body , visited : DenseBitSet :: new_empty (body . basic_blocks . len ()) , worklist } } }}}

macro_rules! preorder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function preorder in module {}", module_path!());
    };
}

mkfn!{
    preorder_introspect!();
    # [doc = " Preorder traversal of a graph."] # [doc = ""] # [doc = " This function creates an iterator over the `Body`'s basic blocks, that"] # [doc = " returns basic blocks in a preorder."] # [doc = ""] # [doc = " See [`Preorder`]'s docs to learn what is preorder traversal."] pub fn preorder < 'a , 'tcx > (body : & 'a Body < 'tcx >) -> Preorder < 'a , 'tcx > { Preorder :: new (body , START_BLOCK) }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > Iterator for Preorder < 'a , 'tcx > { type Item = (BasicBlock , & 'a BasicBlockData < 'tcx >) ; fn next (& mut self) -> Option < (BasicBlock , & 'a BasicBlockData < 'tcx >) > { while let Some (idx) = self . worklist . pop () { if ! self . visited . insert (idx) { continue ; } let data = & self . body [idx] ; if let Some (ref term) = data . terminator { self . worklist . extend (term . successors ()) ; } return Some ((idx , data)) ; } None } fn size_hint (& self) -> (usize , Option < usize >) { let lower = 0 ; let upper = self . body . basic_blocks . len () ; (lower , Some (upper)) } }}}
mkitem!{mkstruct!{# [doc = " Postorder traversal of a graph."] # [doc = ""] # [doc = " Postorder traversal is when each node is visited after all of its successors, except when the"] # [doc = " successor is only reachable by a back-edge. If you are familiar with some basic graph theory,"] # [doc = " then this performs a depth first search and returns nodes in order of completion time."] # [doc = ""] # [doc = ""] # [doc = " ```text"] # [doc = ""] # [doc = "         A"] # [doc = "        / \\"] # [doc = "       /   \\"] # [doc = "      B     C"] # [doc = "       \\   /"] # [doc = "        \\ /"] # [doc = "         D"] # [doc = " ```"] # [doc = ""] # [doc = " A Postorder traversal of this graph is `D B C A` or `D C B A`"] pub struct Postorder < 'a , 'tcx > { basic_blocks : & 'a IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , visited : DenseBitSet < BasicBlock > , visit_stack : Vec < (BasicBlock , Successors < 'a >) > , # [doc = " A non-empty `extra` allows for a precise calculation of the successors."] extra : Option < (TyCtxt < 'tcx > , Instance < 'tcx >) > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Postorder < 'a , 'tcx > { pub fn new (basic_blocks : & 'a IndexSlice < BasicBlock , BasicBlockData < 'tcx > > , root : BasicBlock , extra : Option < (TyCtxt < 'tcx > , Instance < 'tcx >) > ,) -> Postorder < 'a , 'tcx > { let mut po = Postorder { basic_blocks , visited : DenseBitSet :: new_empty (basic_blocks . len ()) , visit_stack : Vec :: new () , extra , } ; po . visit (root) ; po . traverse_successor () ; po } fn visit (& mut self , bb : BasicBlock) { if ! self . visited . insert (bb) { return ; } let data = & self . basic_blocks [bb] ; let successors = if let Some (extra) = self . extra { data . mono_successors (extra . 0 , extra . 1) } else { data . terminator () . successors () } ; self . visit_stack . push ((bb , successors)) ; } fn traverse_successor (& mut self) { while let Some (bb) = self . visit_stack . last_mut () . and_then (| (_ , iter) | iter . next_back ()) { self . visit (bb) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > Iterator for Postorder < '_ , 'tcx > { type Item = BasicBlock ; fn next (& mut self) -> Option < BasicBlock > { let (bb , _) = self . visit_stack . pop () ? ; self . traverse_successor () ; Some (bb) } fn size_hint (& self) -> (usize , Option < usize >) { let lower = self . visit_stack . len () ; let upper = self . basic_blocks . len () ; (lower , Some (upper)) } }}}

macro_rules! postorder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function postorder in module {}", module_path!());
    };
}

mkfn!{
    postorder_introspect!();
    # [doc = " Postorder traversal of a graph."] # [doc = ""] # [doc = " This function creates an iterator over the `Body`'s basic blocks, that:"] # [doc = " - returns basic blocks in a postorder,"] # [doc = " - traverses the `BasicBlocks` CFG cache's reverse postorder backwards, and does not cache the"] # [doc = "   postorder itself."] # [doc = ""] # [doc = " See [`Postorder`]'s docs to learn what is postorder traversal."] pub fn postorder < 'a , 'tcx > (body : & 'a Body < 'tcx > ,) -> impl Iterator < Item = (BasicBlock , & 'a BasicBlockData < 'tcx >) > + ExactSizeIterator + DoubleEndedIterator { reverse_postorder (body) . rev () }
}

macro_rules! mono_reachable_reverse_postorder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mono_reachable_reverse_postorder in module {}", module_path!());
    };
}

mkfn!{
    mono_reachable_reverse_postorder_introspect!();
    pub fn mono_reachable_reverse_postorder < 'a , 'tcx > (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> Vec < BasicBlock > { let mut iter = Postorder :: new (& body . basic_blocks , START_BLOCK , Some ((tcx , instance))) ; let mut items = Vec :: with_capacity (body . basic_blocks . len ()) ; while let Some (block) = iter . next () { items . push (block) ; } items . reverse () ; items }
}

macro_rules! reachable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reachable in module {}", module_path!());
    };
}

mkfn!{
    reachable_introspect!();
    # [doc = " Returns an iterator over all basic blocks reachable from the `START_BLOCK` in no particular"] # [doc = " order."] # [doc = ""] # [doc = " This is clearer than writing `preorder` in cases where the order doesn't matter."] pub fn reachable < 'a , 'tcx > (body : & 'a Body < 'tcx > ,) -> impl 'a + Iterator < Item = (BasicBlock , & 'a BasicBlockData < 'tcx >) > { preorder (body) }
}

macro_rules! reachable_as_bitset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reachable_as_bitset in module {}", module_path!());
    };
}

mkfn!{
    reachable_as_bitset_introspect!();
    # [doc = " Returns a `DenseBitSet` containing all basic blocks reachable from the `START_BLOCK`."] pub fn reachable_as_bitset (body : & Body < '_ >) -> DenseBitSet < BasicBlock > { let mut iter = preorder (body) ; while let Some (_) = iter . next () { } iter . visited }
}

macro_rules! reverse_postorder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reverse_postorder in module {}", module_path!());
    };
}

mkfn!{
    reverse_postorder_introspect!();
    # [doc = " Reverse postorder traversal of a graph."] # [doc = ""] # [doc = " This function creates an iterator over the `Body`'s basic blocks, that:"] # [doc = " - returns basic blocks in a reverse postorder,"] # [doc = " - makes use of the `BasicBlocks` CFG cache's reverse postorder."] # [doc = ""] # [doc = " Reverse postorder is the reverse order of a postorder traversal."] # [doc = " This is different to a preorder traversal and represents a natural"] # [doc = " linearization of control-flow."] # [doc = ""] # [doc = " ```text"] # [doc = ""] # [doc = "         A"] # [doc = "        / \\"] # [doc = "       /   \\"] # [doc = "      B     C"] # [doc = "       \\   /"] # [doc = "        \\ /"] # [doc = "         D"] # [doc = " ```"] # [doc = ""] # [doc = " A reverse postorder traversal of this graph is either `A B C D` or `A C B D`"] # [doc = " Note that for a graph containing no loops (i.e., A DAG), this is equivalent to"] # [doc = " a topological sort."] pub fn reverse_postorder < 'a , 'tcx > (body : & 'a Body < 'tcx > ,) -> impl Iterator < Item = (BasicBlock , & 'a BasicBlockData < 'tcx >) > + ExactSizeIterator + DoubleEndedIterator { body . basic_blocks . reverse_postorder () . iter () . map (| & bb | (bb , & body . basic_blocks [bb])) }
}

macro_rules! mono_reachable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mono_reachable in module {}", module_path!());
    };
}

mkfn!{
    mono_reachable_introspect!();
    # [doc = " Traversal of a [`Body`] that tries to avoid unreachable blocks in a monomorphized [`Instance`]."] # [doc = ""] # [doc = " This is allowed to have false positives; blocks may be visited even if they are not actually"] # [doc = " reachable."] # [doc = ""] # [doc = " Such a traversal is mostly useful because it lets us skip lowering the `false` side"] # [doc = " of `if <T as Trait>::CONST`, as well as [`NullOp::UbChecks`]."] # [doc = ""] # [doc = " [`NullOp::UbChecks`]: rustc_middle::mir::NullOp::UbChecks"] pub fn mono_reachable < 'a , 'tcx > (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> MonoReachable < 'a , 'tcx > { MonoReachable :: new (body , tcx , instance) }
}

macro_rules! mono_reachable_as_bitset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mono_reachable_as_bitset in module {}", module_path!());
    };
}

mkfn!{
    mono_reachable_as_bitset_introspect!();
    # [doc = " [`MonoReachable`] internally accumulates a [`DenseBitSet`] of visited blocks. This is just a"] # [doc = " convenience function to run that traversal then extract its set of reached blocks."] pub fn mono_reachable_as_bitset < 'a , 'tcx > (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> DenseBitSet < BasicBlock > { let mut iter = mono_reachable (body , tcx , instance) ; while let Some (_) = iter . next () { } iter . visited }
}
mkitem!{mkstruct!{pub struct MonoReachable < 'a , 'tcx > { body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , visited : DenseBitSet < BasicBlock > , worklist : DenseBitSet < BasicBlock > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MonoReachable < 'a , 'tcx > { pub fn new (body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > ,) -> MonoReachable < 'a , 'tcx > { let mut worklist = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; worklist . insert (START_BLOCK) ; MonoReachable { body , tcx , instance , visited : DenseBitSet :: new_empty (body . basic_blocks . len ()) , worklist , } } fn add_work (& mut self , blocks : impl IntoIterator < Item = BasicBlock >) { for block in blocks . into_iter () { if ! self . visited . contains (block) { self . worklist . insert (block) ; } } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Iterator for MonoReachable < 'a , 'tcx > { type Item = (BasicBlock , & 'a BasicBlockData < 'tcx >) ; fn next (& mut self) -> Option < (BasicBlock , & 'a BasicBlockData < 'tcx >) > { while let Some (idx) = self . worklist . iter () . next () { self . worklist . remove (idx) ; if ! self . visited . insert (idx) { continue ; } let data = & self . body [idx] ; let targets = data . mono_successors (self . tcx , self . instance) ; self . add_work (targets) ; return Some ((idx , data)) ; } None } }}}