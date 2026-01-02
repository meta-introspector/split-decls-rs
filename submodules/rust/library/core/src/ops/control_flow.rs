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
mkuse!{use crate :: { convert , ops } ;}
mkitem!{mkenum!{#[doc = " Used to tell an operation whether it should exit early or go on as usual."] #[doc = ""] #[doc = " This is used when exposing things (like graph traversals or visitors) where"] #[doc = " you want the user to be able to choose whether to exit early."] #[doc = " Having the enum makes it clearer -- no more wondering \"wait, what did `false`"] #[doc = " mean again?\" -- and allows including a value."] #[doc = ""] #[doc = " Similar to [`Option`] and [`Result`], this enum can be used with the `?` operator"] #[doc = " to return immediately if the [`Break`] variant is present or otherwise continue normally"] #[doc = " with the value inside the [`Continue`] variant."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Early-exiting from [`Iterator::try_for_each`]:"] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " let r = (2..100).try_for_each(|x| {"] #[doc = "     if 403 % x == 0 {"] #[doc = "         return ControlFlow::Break(x)"] #[doc = "     }"] #[doc = ""] #[doc = "     ControlFlow::Continue(())"] #[doc = " });"] #[doc = " assert_eq!(r, ControlFlow::Break(13));"] #[doc = " ```"] #[doc = ""] #[doc = " A basic tree traversal:"] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " pub struct TreeNode<T> {"] #[doc = "     value: T,"] #[doc = "     left: Option<Box<TreeNode<T>>>,"] #[doc = "     right: Option<Box<TreeNode<T>>>,"] #[doc = " }"] #[doc = ""] #[doc = " impl<T> TreeNode<T> {"] #[doc = "     pub fn traverse_inorder<B>(&self, f: &mut impl FnMut(&T) -> ControlFlow<B>) -> ControlFlow<B> {"] #[doc = "         if let Some(left) = &self.left {"] #[doc = "             left.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         f(&self.value)?;"] #[doc = "         if let Some(right) = &self.right {"] #[doc = "             right.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         ControlFlow::Continue(())"] #[doc = "     }"] #[doc = "     fn leaf(value: T) -> Option<Box<TreeNode<T>>> {"] #[doc = "         Some(Box::new(Self { value, left: None, right: None }))"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let node = TreeNode {"] #[doc = "     value: 0,"] #[doc = "     left: TreeNode::leaf(1),"] #[doc = "     right: Some(Box::new(TreeNode {"] #[doc = "         value: -1,"] #[doc = "         left: TreeNode::leaf(5),"] #[doc = "         right: TreeNode::leaf(2),"] #[doc = "     }))"] #[doc = " };"] #[doc = " let mut sum = 0;"] #[doc = ""] #[doc = " let res = node.traverse_inorder(&mut |val| {"] #[doc = "     if *val < 0 {"] #[doc = "         ControlFlow::Break(*val)"] #[doc = "     } else {"] #[doc = "         sum += *val;"] #[doc = "         ControlFlow::Continue(())"] #[doc = "     }"] #[doc = " });"] #[doc = " assert_eq!(res, ControlFlow::Break(-1));"] #[doc = " assert_eq!(sum, 6);"] #[doc = " ```"] #[doc = ""] #[doc = " [`Break`]: ControlFlow::Break"] #[doc = " [`Continue`]: ControlFlow::Continue"] #[stable (feature = "control_flow_enum_type" , since = "1.55.0")] #[rustc_diagnostic_item = "ControlFlow"] #[must_use] #[derive (Copy , Debug , Hash)] #[derive_const (Clone , PartialEq , Eq)] pub enum ControlFlow < B , C = () > { #[doc = " Move on to the next phase of the operation as normal."] #[stable (feature = "control_flow_enum_type" , since = "1.55.0")] #[lang = "Continue"] Continue (C) , #[doc = " Exit the operation without running subsequent phases."] #[stable (feature = "control_flow_enum_type" , since = "1.55.0")] #[lang = "Break"] Break (B) , }}}
mkitem!{#[unstable (feature = "try_trait_v2" , issue = "84277" , old_name = "try_trait")] #[rustc_const_unstable (feature = "const_try" , issue = "74935")] impl < B , C > const ops :: Try for ControlFlow < B , C > { type Output = C ; type Residual = ControlFlow < B , convert :: Infallible >; #[inline] fn from_output (output : Self :: Output) -> Self { ControlFlow :: Continue (output) } #[inline] fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { ControlFlow :: Continue (c) => ControlFlow :: Continue (c) , ControlFlow :: Break (b) => ControlFlow :: Break (ControlFlow :: Break (b)) , } } }}
mkitem!{#[unstable (feature = "try_trait_v2" , issue = "84277" , old_name = "try_trait")] #[rustc_const_unstable (feature = "const_try" , issue = "74935")] impl < B , C > const ops :: FromResidual < ControlFlow < B , convert :: Infallible >> for ControlFlow < B , C > { #[inline] fn from_residual (residual : ControlFlow < B , convert :: Infallible >) -> Self { match residual { ControlFlow :: Break (b) => ControlFlow :: Break (b) , } } }}
mkitem!{mkimpl!{#[unstable (feature = "try_trait_v2_residual" , issue = "91285")] impl < B , C > ops :: Residual < C > for ControlFlow < B , convert :: Infallible > { type TryType = ControlFlow < B , C > ; }}}
mkitem!{mkimpl!{impl < B , C > ControlFlow < B , C > { #[doc = " Returns `true` if this is a `Break` variant."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " assert!(ControlFlow::<&str, i32>::Break(\"Stop right there!\").is_break());"] #[doc = " assert!(!ControlFlow::<&str, i32>::Continue(3).is_break());"] #[doc = " ```"] #[inline] #[stable (feature = "control_flow_enum_is" , since = "1.59.0")] pub fn is_break (& self) -> bool { matches ! (* self , ControlFlow :: Break (_)) } #[doc = " Returns `true` if this is a `Continue` variant."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " assert!(!ControlFlow::<&str, i32>::Break(\"Stop right there!\").is_continue());"] #[doc = " assert!(ControlFlow::<&str, i32>::Continue(3).is_continue());"] #[doc = " ```"] #[inline] #[stable (feature = "control_flow_enum_is" , since = "1.59.0")] pub fn is_continue (& self) -> bool { matches ! (* self , ControlFlow :: Continue (_)) } #[doc = " Converts the `ControlFlow` into an `Option` which is `Some` if the"] #[doc = " `ControlFlow` was `Break` and `None` otherwise."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " assert_eq!(ControlFlow::<&str, i32>::Break(\"Stop right there!\").break_value(), Some(\"Stop right there!\"));"] #[doc = " assert_eq!(ControlFlow::<&str, i32>::Continue(3).break_value(), None);"] #[doc = " ```"] #[inline] #[stable (feature = "control_flow_enum" , since = "1.83.0")] pub fn break_value (self) -> Option < B > { match self { ControlFlow :: Continue (..) => None , ControlFlow :: Break (x) => Some (x) , } } #[doc = " Converts the `ControlFlow` into an `Result` which is `Ok` if the"] #[doc = " `ControlFlow` was `Break` and `Err` if otherwise."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(control_flow_ok)]"] #[doc = ""] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " struct TreeNode<T> {"] #[doc = "     value: T,"] #[doc = "     left: Option<Box<TreeNode<T>>>,"] #[doc = "     right: Option<Box<TreeNode<T>>>,"] #[doc = " }"] #[doc = ""] #[doc = " impl<T> TreeNode<T> {"] #[doc = "     fn find<'a>(&'a self, mut predicate: impl FnMut(&T) -> bool) -> Result<&'a T, ()> {"] #[doc = "         let mut f = |t: &'a T| -> ControlFlow<&'a T> {"] #[doc = "             if predicate(t) {"] #[doc = "                 ControlFlow::Break(t)"] #[doc = "             } else {"] #[doc = "                 ControlFlow::Continue(())"] #[doc = "             }"] #[doc = "         };"] #[doc = ""] #[doc = "         self.traverse_inorder(&mut f).break_ok()"] #[doc = "     }"] #[doc = ""] #[doc = "     fn traverse_inorder<'a, B>("] #[doc = "         &'a self,"] #[doc = "         f: &mut impl FnMut(&'a T) -> ControlFlow<B>,"] #[doc = "     ) -> ControlFlow<B> {"] #[doc = "         if let Some(left) = &self.left {"] #[doc = "             left.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         f(&self.value)?;"] #[doc = "         if let Some(right) = &self.right {"] #[doc = "             right.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         ControlFlow::Continue(())"] #[doc = "     }"] #[doc = ""] #[doc = "     fn leaf(value: T) -> Option<Box<TreeNode<T>>> {"] #[doc = "         Some(Box::new(Self {"] #[doc = "             value,"] #[doc = "             left: None,"] #[doc = "             right: None,"] #[doc = "         }))"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let node = TreeNode {"] #[doc = "     value: 0,"] #[doc = "     left: TreeNode::leaf(1),"] #[doc = "     right: Some(Box::new(TreeNode {"] #[doc = "         value: -1,"] #[doc = "         left: TreeNode::leaf(5),"] #[doc = "         right: TreeNode::leaf(2),"] #[doc = "     })),"] #[doc = " };"] #[doc = ""] #[doc = " let res = node.find(|val: &i32| *val > 3);"] #[doc = " assert_eq!(res, Ok(&5));"] #[doc = " ```"] #[inline] #[unstable (feature = "control_flow_ok" , issue = "140266")] pub fn break_ok (self) -> Result < B , C > { match self { ControlFlow :: Continue (c) => Err (c) , ControlFlow :: Break (b) => Ok (b) , } } #[doc = " Maps `ControlFlow<B, C>` to `ControlFlow<T, C>` by applying a function"] #[doc = " to the break value in case it exists."] #[inline] #[stable (feature = "control_flow_enum" , since = "1.83.0")] pub fn map_break < T > (self , f : impl FnOnce (B) -> T) -> ControlFlow < T , C > { match self { ControlFlow :: Continue (x) => ControlFlow :: Continue (x) , ControlFlow :: Break (x) => ControlFlow :: Break (f (x)) , } } #[doc = " Converts the `ControlFlow` into an `Option` which is `Some` if the"] #[doc = " `ControlFlow` was `Continue` and `None` otherwise."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " assert_eq!(ControlFlow::<&str, i32>::Break(\"Stop right there!\").continue_value(), None);"] #[doc = " assert_eq!(ControlFlow::<&str, i32>::Continue(3).continue_value(), Some(3));"] #[doc = " ```"] #[inline] #[stable (feature = "control_flow_enum" , since = "1.83.0")] pub fn continue_value (self) -> Option < C > { match self { ControlFlow :: Continue (x) => Some (x) , ControlFlow :: Break (..) => None , } } #[doc = " Converts the `ControlFlow` into an `Result` which is `Ok` if the"] #[doc = " `ControlFlow` was `Continue` and `Err` if otherwise."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(control_flow_ok)]"] #[doc = ""] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " struct TreeNode<T> {"] #[doc = "     value: T,"] #[doc = "     left: Option<Box<TreeNode<T>>>,"] #[doc = "     right: Option<Box<TreeNode<T>>>,"] #[doc = " }"] #[doc = ""] #[doc = " impl<T> TreeNode<T> {"] #[doc = "     fn validate<B>(&self, f: &mut impl FnMut(&T) -> ControlFlow<B>) -> Result<(), B> {"] #[doc = "         self.traverse_inorder(f).continue_ok()"] #[doc = "     }"] #[doc = ""] #[doc = "     fn traverse_inorder<B>(&self, f: &mut impl FnMut(&T) -> ControlFlow<B>) -> ControlFlow<B> {"] #[doc = "         if let Some(left) = &self.left {"] #[doc = "             left.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         f(&self.value)?;"] #[doc = "         if let Some(right) = &self.right {"] #[doc = "             right.traverse_inorder(f)?;"] #[doc = "         }"] #[doc = "         ControlFlow::Continue(())"] #[doc = "     }"] #[doc = ""] #[doc = "     fn leaf(value: T) -> Option<Box<TreeNode<T>>> {"] #[doc = "         Some(Box::new(Self {"] #[doc = "             value,"] #[doc = "             left: None,"] #[doc = "             right: None,"] #[doc = "         }))"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc = " let node = TreeNode {"] #[doc = "     value: 0,"] #[doc = "     left: TreeNode::leaf(1),"] #[doc = "     right: Some(Box::new(TreeNode {"] #[doc = "         value: -1,"] #[doc = "         left: TreeNode::leaf(5),"] #[doc = "         right: TreeNode::leaf(2),"] #[doc = "     })),"] #[doc = " };"] #[doc = ""] #[doc = " let res = node.validate(&mut |val| {"] #[doc = "     if *val < 0 {"] #[doc = "         return ControlFlow::Break(\"negative value detected\");"] #[doc = "     }"] #[doc = ""] #[doc = "     if *val > 4 {"] #[doc = "         return ControlFlow::Break(\"too big value detected\");"] #[doc = "     }"] #[doc = ""] #[doc = "     ControlFlow::Continue(())"] #[doc = " });"] #[doc = " assert_eq!(res, Err(\"too big value detected\"));"] #[doc = " ```"] #[inline] #[unstable (feature = "control_flow_ok" , issue = "140266")] pub fn continue_ok (self) -> Result < C , B > { match self { ControlFlow :: Continue (c) => Ok (c) , ControlFlow :: Break (b) => Err (b) , } } #[doc = " Maps `ControlFlow<B, C>` to `ControlFlow<B, T>` by applying a function"] #[doc = " to the continue value in case it exists."] #[inline] #[stable (feature = "control_flow_enum" , since = "1.83.0")] pub fn map_continue < T > (self , f : impl FnOnce (C) -> T) -> ControlFlow < B , T > { match self { ControlFlow :: Continue (x) => ControlFlow :: Continue (f (x)) , ControlFlow :: Break (x) => ControlFlow :: Break (x) , } } }}}
mkitem!{mkimpl!{impl < T > ControlFlow < T , T > { #[doc = " Extracts the value `T` that is wrapped by `ControlFlow<T, T>`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(control_flow_into_value)]"] #[doc = " use std::ops::ControlFlow;"] #[doc = ""] #[doc = " assert_eq!(ControlFlow::<i32, i32>::Break(1024).into_value(), 1024);"] #[doc = " assert_eq!(ControlFlow::<i32, i32>::Continue(512).into_value(), 512);"] #[doc = " ```"] #[unstable (feature = "control_flow_into_value" , issue = "137461")] #[rustc_allow_const_fn_unstable (const_precise_live_drops)] pub const fn into_value (self) -> T { match self { ControlFlow :: Continue (x) | ControlFlow :: Break (x) => x , } } }}}
mkitem!{mkimpl!{#[doc = " These are used only as part of implementing the iterator adapters."] #[doc = " They have mediocre names and non-obvious semantics, so aren't"] #[doc = " currently on a path to potential stabilization."] impl < R : ops :: Try > ControlFlow < R , R :: Output > { #[doc = " Creates a `ControlFlow` from any type implementing `Try`."] #[inline] pub (crate) fn from_try (r : R) -> Self { match R :: branch (r) { ControlFlow :: Continue (v) => ControlFlow :: Continue (v) , ControlFlow :: Break (v) => ControlFlow :: Break (R :: from_residual (v)) , } } #[doc = " Converts a `ControlFlow` into any type implementing `Try`."] #[inline] pub (crate) fn into_try (self) -> R { match self { ControlFlow :: Continue (v) => R :: from_output (v) , ControlFlow :: Break (v) => v , } } }}}