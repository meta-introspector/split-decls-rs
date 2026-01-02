mkuse!{use std :: cell :: { Cell , RefCell } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_public_bridge :: context :: CompilerCtxt ;}
mkuse!{use rustc_public_bridge :: { Bridge , Container , Tables } ;}
mkuse!{use rustc_span :: def_id :: CrateNum ;}
mkuse!{use scoped_tls :: scoped_thread_local ;}
mkuse!{use crate :: Error ;}
mkuse!{use crate :: unstable :: { RustcInternal , Stable } ;}
mkmod!{pretty, { 
                getname!(pretty);
                getsrc!(pretty);
                getpath!(pretty);
                get_deps!(pretty);
                get_crates!(pretty);
                mkinclude!(pretty);
                 
            }}

macro_rules! stable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stable in module {}", module_path!());
    };
}

mkfn!{
    stable_introspect!();
    # [doc = " Convert an internal Rust compiler item into its stable counterpart, if one exists."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is unstable, and its behavior may change at any point."] # [doc = " E.g.: Items that were previously supported, may no longer be supported, or its translation may"] # [doc = " change."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if rustc_public has not been properly initialized."] pub fn stable < 'tcx , S : Stable < 'tcx > > (item : S) -> S :: T { with_container (| tables , cx | item . stable (tables , cx)) }
}

macro_rules! internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function internal in module {}", module_path!());
    };
}

mkfn!{
    internal_introspect!();
    # [doc = " Convert a stable item into its internal Rust compiler counterpart, if one exists."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is unstable, and it's behavior may change at any point."] # [doc = " Not every stable item can be converted to an internal one."] # [doc = " Furthermore, items that were previously supported, may no longer be supported in newer versions."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if rustc_public has not been properly initialized."] pub fn internal < 'tcx , S > (tcx : TyCtxt < 'tcx > , item : S) -> S :: T < 'tcx > where S : RustcInternal , { with_container (| tables , _ | item . internal (tables , tcx)) }
}

macro_rules! crate_num_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crate_num in module {}", module_path!());
    };
}

mkfn!{
    crate_num_introspect!();
    pub fn crate_num (item : & crate :: Crate) -> CrateNum { item . id . into () }
}
mkitem!{scoped_thread_local ! (static TLV : Cell <* const () >) ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub (crate) fn init < 'tcx , F , T , B : Bridge > (container : & Container < 'tcx , B > , f : F) -> T where F : FnOnce () -> T , { assert ! (! TLV . is_set ()) ; let ptr = container as * const _ as * const () ; TLV . set (& Cell :: new (ptr) , | | f ()) }
}

macro_rules! with_container_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_container in module {}", module_path!());
    };
}

mkfn!{
    with_container_introspect!();
    # [doc = " Loads the current context and calls a function with it."] # [doc = " Do not nest these, as that will ICE."] pub (crate) fn with_container < R , B : Bridge > (f : impl for < 'tcx > FnOnce (& mut Tables < 'tcx , B > , & CompilerCtxt < 'tcx , B >) -> R ,) -> R { assert ! (TLV . is_set ()) ; TLV . with (| tlv | { let ptr = tlv . get () ; assert ! (! ptr . is_null ()) ; let container = ptr as * const Container < '_ , B > ; let mut tables = unsafe { (* container) . tables . borrow_mut () } ; let cx = unsafe { (* container) . cx . borrow () } ; f (& mut * tables , & * cx) }) }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run < F , T > (tcx : TyCtxt < '_ > , f : F) -> Result < T , Error > where F : FnOnce () -> T , { let compiler_cx = RefCell :: new (CompilerCtxt :: new (tcx)) ; let container = Container { tables : RefCell :: new (Tables :: default ()) , cx : compiler_cx } ; crate :: compiler_interface :: run (& container , | | init (& container , f)) }
}
mkitem!{# [doc = " Instantiate and run the compiler with the provided arguments and callback."] # [doc = ""] # [doc = " The callback will be invoked after the compiler ran all its analyses, but before code generation."] # [doc = " Note that this macro accepts two different formats for the callback:"] # [doc = " 1. An ident that resolves to a function that accepts no argument and returns `ControlFlow<B, C>`"] # [doc = " ```ignore(needs-extern-crate)"] # [doc = " # extern crate rustc_driver;"] # [doc = " # extern crate rustc_interface;"] # [doc = " # extern crate rustc_middle;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate rustc_public;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #   use std::ops::ControlFlow;"] # [doc = " #   use rustc_public::CompilerError;"] # [doc = "     fn analyze_code() -> ControlFlow<(), ()> {"] # [doc = "         // Your code goes in here."] # [doc = " #       ControlFlow::Continue(())"] # [doc = "     }"] # [doc = " #   let args = &[\"--verbose\".to_string()];"] # [doc = "     let result = run!(args, analyze_code);"] # [doc = " #   assert_eq!(result, Err(CompilerError::Skipped))"] # [doc = " # }"] # [doc = " ```"] # [doc = " 2. A closure expression:"] # [doc = " ```ignore(needs-extern-crate)"] # [doc = " # extern crate rustc_driver;"] # [doc = " # extern crate rustc_interface;"] # [doc = " # extern crate rustc_middle;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate rustc_public;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #   use std::ops::ControlFlow;"] # [doc = " #   use rustc_public::CompilerError;"] # [doc = "     fn analyze_code(extra_args: Vec<String>) -> ControlFlow<(), ()> {"] # [doc = " #       let _ = extra_args;"] # [doc = "         // Your code goes in here."] # [doc = " #       ControlFlow::Continue(())"] # [doc = "     }"] # [doc = " #   let args = &[\"--verbose\".to_string()];"] # [doc = " #   let extra_args = vec![];"] # [doc = "     let result = run!(args, || analyze_code(extra_args));"] # [doc = " #   assert_eq!(result, Err(CompilerError::Skipped))"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! run { ($ args : expr , $ callback_fn : ident) => { $ crate :: run_driver ! ($ args , || $ callback_fn ()) } ; ($ args : expr , $ callback : expr) => { $ crate :: run_driver ! ($ args , $ callback) } ; }}
mkitem!{# [doc = " Instantiate and run the compiler with the provided arguments and callback."] # [doc = ""] # [doc = " This is similar to `run` but it invokes the callback with the compiler's `TyCtxt`,"] # [doc = " which can be used to invoke internal APIs."] # [macro_export] macro_rules ! run_with_tcx { ($ args : expr , $ callback_fn : ident) => { $ crate :: run_driver ! ($ args , | tcx | $ callback_fn (tcx) , with_tcx) } ; ($ args : expr , $ callback : expr) => { $ crate :: run_driver ! ($ args , $ callback , with_tcx) } ; }}
mkitem!{# [doc = " Optionally include an ident. This is needed due to macro hygiene."] # [macro_export] # [doc (hidden)] macro_rules ! optional { (with_tcx $ ident : ident) => { $ ident } ; }}
mkitem!{# [doc = " Prefer using [run!] and [run_with_tcx] instead."] # [doc = ""] # [doc = " This macro implements the instantiation of a rustc_public driver, and it will invoke"] # [doc = " the given callback after the compiler analyses."] # [doc = ""] # [doc = " The third argument determines whether the callback requires `tcx` as an argument."] # [macro_export] # [doc (hidden)] macro_rules ! run_driver { ($ args : expr , $ callback : expr $ (, $ with_tcx : ident) ?) => { { use rustc_driver :: { Callbacks , Compilation , run_compiler } ; use rustc_middle :: ty :: TyCtxt ; use rustc_interface :: interface ; use rustc_public :: rustc_internal ; use rustc_public :: CompilerError ; use std :: ops :: ControlFlow ; pub struct RustcPublic < B = () , C = () , F = fn ($ ($ crate :: optional ! ($ with_tcx TyCtxt)) ?) -> ControlFlow < B , C >> where B : Send , C : Send , F : FnOnce ($ ($ crate :: optional ! ($ with_tcx TyCtxt)) ?) -> ControlFlow < B , C > + Send , { callback : Option < F >, result : Option < ControlFlow < B , C >>, } impl < B , C , F > RustcPublic < B , C , F > where B : Send , C : Send , F : FnOnce ($ ($ crate :: optional ! ($ with_tcx TyCtxt)) ?) -> ControlFlow < B , C > + Send , { # [doc = " Creates a new `RustcPublic` instance, with given test_function and arguments."] pub fn new (callback : F) -> Self { RustcPublic { callback : Some (callback) , result : None } } # [doc = " Runs the compiler against given target and tests it with `test_function`"] pub fn run (& mut self , args : & [String]) -> Result < C , CompilerError < B >> { let compiler_result = rustc_driver :: catch_fatal_errors (|| -> interface :: Result ::< () > { run_compiler (& args , self) ; Ok (()) }) ; match (compiler_result , self . result . take ()) { (Ok (Ok (())) , Some (ControlFlow :: Continue (value))) => Ok (value) , (Ok (Ok (())) , Some (ControlFlow :: Break (value))) => { Err (CompilerError :: Interrupted (value)) } (Ok (Ok (_)) , None) => Err (CompilerError :: Skipped) , (Ok (Err (_)) , _) | (Err (_) , _) => Err (CompilerError :: Failed) , } } } impl < B , C , F > Callbacks for RustcPublic < B , C , F > where B : Send , C : Send , F : FnOnce ($ ($ crate :: optional ! ($ with_tcx TyCtxt)) ?) -> ControlFlow < B , C > + Send , { # [doc = " Called after analysis. Return value instructs the compiler whether to"] # [doc = " continue the compilation afterwards (defaults to `Compilation::Continue`)"] fn after_analysis <'tcx > (& mut self , _compiler : & interface :: Compiler , tcx : TyCtxt <'tcx >,) -> Compilation { if let Some (callback) = self . callback . take () { rustc_internal :: run (tcx , || { self . result = Some (callback ($ ($ crate :: optional ! ($ with_tcx tcx)) ?)) ; }) . unwrap () ; if self . result . as_ref () . is_some_and (| val | val . is_continue ()) { Compilation :: Continue } else { Compilation :: Stop } } else { Compilation :: Continue } } } RustcPublic :: new ($ callback) . run ($ args) } } ; }}