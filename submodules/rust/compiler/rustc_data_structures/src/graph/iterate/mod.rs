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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use super :: { DirectedGraph , StartNode , Successors } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! post_order_from_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function post_order_from in module {}", module_path!());
    };
}

mkfn!{
    post_order_from_introspect!();
    pub fn post_order_from < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { post_order_from_to (graph , start_node , None) }
}

macro_rules! post_order_from_to_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function post_order_from_to in module {}", module_path!());
    };
}

mkfn!{
    post_order_from_to_introspect!();
    pub fn post_order_from_to < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node , end_node : Option < G :: Node > ,) -> Vec < G :: Node > { let mut visited : IndexVec < G :: Node , bool > = IndexVec :: from_elem_n (false , graph . num_nodes ()) ; let mut result : Vec < G :: Node > = Vec :: with_capacity (graph . num_nodes ()) ; if let Some (end_node) = end_node { visited [end_node] = true ; } post_order_walk (graph , start_node , & mut result , & mut visited) ; result }
}

macro_rules! post_order_walk_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function post_order_walk in module {}", module_path!());
    };
}

mkfn!{
    post_order_walk_introspect!();
    fn post_order_walk < G : DirectedGraph + Successors > (graph : & G , node : G :: Node , result : & mut Vec < G :: Node > , visited : & mut IndexSlice < G :: Node , bool > ,) { struct PostOrderFrame < Node , Iter > { node : Node , iter : Iter , } if visited [node] { return ; } let mut stack = vec ! [PostOrderFrame { node , iter : graph . successors (node) }] ; 'recurse : while let Some (frame) = stack . last_mut () { let node = frame . node ; visited [node] = true ; for successor in frame . iter . by_ref () { if ! visited [successor] { stack . push (PostOrderFrame { node : successor , iter : graph . successors (successor) }) ; continue 'recurse ; } } let _ = stack . pop () ; result . push (node) ; } }
}

macro_rules! reverse_post_order_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reverse_post_order in module {}", module_path!());
    };
}

mkfn!{
    reverse_post_order_introspect!();
    pub fn reverse_post_order < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { let mut vec = post_order_from (graph , start_node) ; vec . reverse () ; vec }
}
mkitem!{mkstruct!{#[doc = " A \"depth-first search\" iterator for a directed graph."] pub struct DepthFirstSearch < G > where G : DirectedGraph + Successors , { graph : G , stack : Vec < G :: Node > , visited : DenseBitSet < G :: Node > , }}}
mkitem!{mkimpl!{impl < G > DepthFirstSearch < G > where G : DirectedGraph + Successors , { pub fn new (graph : G) -> Self { Self { stack : vec ! [] , visited : DenseBitSet :: new_empty (graph . num_nodes ()) , graph } } #[doc = " Version of `push_start_node` that is convenient for chained"] #[doc = " use."] pub fn with_start_node (mut self , start_node : G :: Node) -> Self { self . push_start_node (start_node) ; self } #[doc = " Pushes another start node onto the stack. If the node"] #[doc = " has not already been visited, then you will be able to"] #[doc = " walk its successors (and so forth) after the current"] #[doc = " contents of the stack are drained. If multiple start nodes"] #[doc = " are added into the walk, then their mutual successors"] #[doc = " will all be walked. You can use this method once the"] #[doc = " iterator has been completely drained to add additional"] #[doc = " start nodes."] pub fn push_start_node (& mut self , start_node : G :: Node) { if self . visited . insert (start_node) { self . stack . push (start_node) ; } } #[doc = " Searches all nodes reachable from the current start nodes."] #[doc = " This is equivalent to just invoke `next` repeatedly until"] #[doc = " you get a `None` result."] pub fn complete_search (& mut self) { for _ in self . by_ref () { } } #[doc = " Returns true if node has been visited thus far."] #[doc = " A node is considered \"visited\" once it is pushed"] #[doc = " onto the internal stack; it may not yet have been yielded"] #[doc = " from the iterator. This method is best used after"] #[doc = " the iterator is completely drained."] pub fn visited (& self , node : G :: Node) -> bool { self . visited . contains (node) } #[doc = " Returns a reference to the set of nodes that have been visited, with"] #[doc = " the same caveats as [`Self::visited`]."] #[doc = ""] #[doc = " When incorporating the visited nodes into another bitset, using bulk"] #[doc = " operations like `union` or `intersect` can be more efficient than"] #[doc = " processing each node individually."] pub fn visited_set (& self) -> & DenseBitSet < G :: Node > { & self . visited } }}}
mkitem!{mkimpl!{impl < G > std :: fmt :: Debug for DepthFirstSearch < G > where G : DirectedGraph + Successors , { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut f = fmt . debug_set () ; for n in self . visited . iter () { f . entry (& n) ; } f . finish () } }}}
mkitem!{mkimpl!{impl < G > Iterator for DepthFirstSearch < G > where G : DirectedGraph + Successors , { type Item = G :: Node ; fn next (& mut self) -> Option < G :: Node > { let DepthFirstSearch { stack , visited , graph } = self ; let n = stack . pop () ? ; stack . extend (graph . successors (n) . filter (| & m | visited . insert (m))) ; Some (n) } }}}
mkitem!{mkenum!{#[doc = " The status of a node in the depth-first search."] #[doc = ""] #[doc = " See the documentation of `TriColorDepthFirstSearch` to see how a node's status is updated"] #[doc = " during DFS."] #[derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum NodeStatus { #[doc = " This node has been examined by the depth-first search but is not yet `Settled`."] #[doc = ""] #[doc = " Also referred to as \"gray\" or \"discovered\" nodes in [CLR]."] #[doc = ""] #[doc = " [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms"] Visited , #[doc = " This node and all nodes reachable from it have been examined by the depth-first search."] #[doc = ""] #[doc = " Also referred to as \"black\" or \"finished\" nodes in [CLR]."] #[doc = ""] #[doc = " [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms"] Settled , }}}
mkitem!{mkstruct!{struct Event < N > { node : N , becomes : NodeStatus , }}}
mkitem!{mkstruct!{#[doc = " A depth-first search that also tracks when all successors of a node have been examined."] #[doc = ""] #[doc = " This is based on the DFS described in [Introduction to Algorithms (1st ed.)][CLR], hereby"] #[doc = " referred to as **CLR**. However, we use the terminology in [`NodeStatus`] above instead of"] #[doc = " \"discovered\"/\"finished\" or \"white\"/\"grey\"/\"black\". Each node begins the search with no status,"] #[doc = " becomes `Visited` when it is first examined by the DFS and is `Settled` when all nodes"] #[doc = " reachable from it have been examined. This allows us to differentiate between \"tree\", \"back\""] #[doc = " and \"forward\" edges (see [`TriColorVisitor::node_examined`])."] #[doc = ""] #[doc = " Unlike the pseudocode in [CLR], this implementation is iterative and does not use timestamps."] #[doc = " We accomplish this by storing `Event`s on the stack that result in a (possible) state change"] #[doc = " for each node. A `Visited` event signifies that we should examine this node if it has not yet"] #[doc = " been `Visited` or `Settled`. When a node is examined for the first time, we mark it as"] #[doc = " `Visited` and push a `Settled` event for it on stack followed by `Visited` events for all of"] #[doc = " its predecessors, scheduling them for examination. Multiple `Visited` events for a single node"] #[doc = " may exist on the stack simultaneously if a node has multiple predecessors, but only one"] #[doc = " `Settled` event will ever be created for each node. After all `Visited` events for a node's"] #[doc = " successors have been popped off the stack (as well as any new events triggered by visiting"] #[doc = " those successors), we will pop off that node's `Settled` event."] #[doc = ""] #[doc = " [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms"] pub struct TriColorDepthFirstSearch < 'graph , G > where G : ? Sized + DirectedGraph + Successors , { graph : & 'graph G , stack : Vec < Event < G :: Node > > , visited : DenseBitSet < G :: Node > , settled : DenseBitSet < G :: Node > , }}}
mkitem!{mkimpl!{impl < 'graph , G > TriColorDepthFirstSearch < 'graph , G > where G : ? Sized + DirectedGraph + Successors , { pub fn new (graph : & 'graph G) -> Self { TriColorDepthFirstSearch { graph , stack : vec ! [] , visited : DenseBitSet :: new_empty (graph . num_nodes ()) , settled : DenseBitSet :: new_empty (graph . num_nodes ()) , } } #[doc = " Performs a depth-first search, starting from the given `root`."] #[doc = ""] #[doc = " This won't visit nodes that are not reachable from `root`."] pub fn run_from < V > (mut self , root : G :: Node , visitor : & mut V) -> Option < V :: BreakVal > where V : TriColorVisitor < G > , { use NodeStatus :: { Settled , Visited } ; self . stack . push (Event { node : root , becomes : Visited }) ; loop { match self . stack . pop () ? { Event { node , becomes : Settled } => { let not_previously_settled = self . settled . insert (node) ; assert ! (not_previously_settled , "A node should be settled exactly once") ; if let ControlFlow :: Break (val) = visitor . node_settled (node) { return Some (val) ; } } Event { node , becomes : Visited } => { let not_previously_visited = self . visited . insert (node) ; let prior_status = if not_previously_visited { None } else if self . settled . contains (node) { Some (Settled) } else { Some (Visited) } ; if let ControlFlow :: Break (val) = visitor . node_examined (node , prior_status) { return Some (val) ; } if prior_status . is_some () { continue ; } self . stack . push (Event { node , becomes : Settled }) ; for succ in self . graph . successors (node) { if ! visitor . ignore_edge (node , succ) { self . stack . push (Event { node : succ , becomes : Visited }) ; } } } } } } }}}
mkitem!{mkimpl!{impl < G > TriColorDepthFirstSearch < '_ , G > where G : ? Sized + DirectedGraph + Successors + StartNode , { #[doc = " Performs a depth-first search, starting from `G::start_node()`."] #[doc = ""] #[doc = " This won't visit nodes that are not reachable from the start node."] pub fn run_from_start < V > (self , visitor : & mut V) -> Option < V :: BreakVal > where V : TriColorVisitor < G > , { let root = self . graph . start_node () ; self . run_from (root , visitor) } }}}
mkitem!{mktrait!{#[doc = " What to do when a node is examined or becomes `Settled` during DFS."] pub trait TriColorVisitor < G > where G : ? Sized + DirectedGraph , { #[doc = " The value returned by this search."] type BreakVal ; #[doc = " Called when a node is examined by the depth-first search."] #[doc = ""] #[doc = " By checking the value of `prior_status`, this visitor can determine whether the edge"] #[doc = " leading to this node was a tree edge (`None`), forward edge (`Some(Settled)`) or back edge"] #[doc = " (`Some(Visited)`). For a full explanation of each edge type, see the \"Depth-first Search\""] #[doc = " chapter in [CLR] or [wikipedia]."] #[doc = ""] #[doc = " If you want to know *both* nodes linked by each edge, you'll need to modify"] #[doc = " `TriColorDepthFirstSearch` to store a `source` node for each `Visited` event."] #[doc = ""] #[doc = " [wikipedia]: https://en.wikipedia.org/wiki/Depth-first_search#Output_of_a_depth-first_search"] #[doc = " [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms"] fn node_examined (& mut self , _node : G :: Node , _prior_status : Option < NodeStatus > ,) -> ControlFlow < Self :: BreakVal > { ControlFlow :: Continue (()) } #[doc = " Called after all nodes reachable from this one have been examined."] fn node_settled (& mut self , _node : G :: Node) -> ControlFlow < Self :: BreakVal > { ControlFlow :: Continue (()) } #[doc = " Behave as if no edges exist from `source` to `target`."] fn ignore_edge (& mut self , _source : G :: Node , _target : G :: Node) -> bool { false } }}}
mkitem!{mkstruct!{#[doc = " This `TriColorVisitor` looks for back edges in a graph, which indicate that a cycle exists."] pub struct CycleDetector ;}}
mkitem!{mkimpl!{impl < G > TriColorVisitor < G > for CycleDetector where G : ? Sized + DirectedGraph , { type BreakVal = () ; fn node_examined (& mut self , _node : G :: Node , prior_status : Option < NodeStatus > ,) -> ControlFlow < Self :: BreakVal > { match prior_status { Some (NodeStatus :: Visited) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }}}