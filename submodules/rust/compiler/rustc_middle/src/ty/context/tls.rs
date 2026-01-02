mkuse!{use std :: { mem , ptr } ;}
mkuse!{use rustc_data_structures :: sync ;}
mkuse!{use super :: { GlobalCtxt , TyCtxt } ;}
mkuse!{use crate :: dep_graph :: TaskDepsRef ;}
mkuse!{use crate :: query :: plumbing :: QueryJobId ;}
mkitem!{mkstruct!{# [doc = " This is the implicit state of rustc. It contains the current"] # [doc = " `TyCtxt` and query. It is updated when creating a local interner or"] # [doc = " executing a new query. Whenever there's a `TyCtxt` value available"] # [doc = " you should also have access to an `ImplicitCtxt` through the functions"] # [doc = " in this module."] # [derive (Clone)] pub struct ImplicitCtxt < 'a , 'tcx > { # [doc = " The current `TyCtxt`."] pub tcx : TyCtxt < 'tcx > , # [doc = " The current query job, if any. This is updated by `JobOwner::start` in"] # [doc = " `ty::query::plumbing` when executing a query."] pub query : Option < QueryJobId > , # [doc = " Used to prevent queries from calling too deeply."] pub query_depth : usize , # [doc = " The current dep graph task. This is used to add dependencies to queries"] # [doc = " when executing them."] pub task_deps : TaskDepsRef < 'a > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ImplicitCtxt < 'a , 'tcx > { pub fn new (gcx : & 'tcx GlobalCtxt < 'tcx >) -> Self { let tcx = TyCtxt { gcx } ; ImplicitCtxt { tcx , query : None , query_depth : 0 , task_deps : TaskDepsRef :: Ignore } } }}}
mkuse!{use rustc_thread_pool :: tlv :: TLV ;}

macro_rules! erase_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erase in module {}", module_path!());
    };
}

mkfn!{
    erase_introspect!();
    # [inline] fn erase (context : & ImplicitCtxt < '_ , '_ >) -> * const () { context as * const _ as * const () }
}

macro_rules! downcast_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function downcast in module {}", module_path!());
    };
}

mkfn!{
    downcast_introspect!();
    # [inline] unsafe fn downcast < 'a , 'tcx > (context : * const ()) -> & 'a ImplicitCtxt < 'a , 'tcx > { unsafe { & * (context as * const ImplicitCtxt < 'a , 'tcx >) } }
}

macro_rules! enter_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enter_context in module {}", module_path!());
    };
}

mkfn!{
    enter_context_introspect!();
    # [doc = " Sets `context` as the new current `ImplicitCtxt` for the duration of the function `f`."] # [inline] pub fn enter_context < 'a , 'tcx , F , R > (context : & ImplicitCtxt < 'a , 'tcx > , f : F) -> R where F : FnOnce () -> R , { TLV . with (| tlv | { let old = tlv . replace (erase (context)) ; let _reset = rustc_data_structures :: defer (move | | tlv . set (old)) ; f () }) }
}

macro_rules! with_context_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_context_opt in module {}", module_path!());
    };
}

mkfn!{
    with_context_opt_introspect!();
    # [doc = " Allows access to the current `ImplicitCtxt` in a closure if one is available."] # [inline] # [track_caller] pub fn with_context_opt < F , R > (f : F) -> R where F : for < 'a , 'tcx > FnOnce (Option < & ImplicitCtxt < 'a , 'tcx > >) -> R , { let context = TLV . get () ; if context . is_null () { f (None) } else { sync :: assert_dyn_sync :: < ImplicitCtxt < '_ , '_ > > () ; unsafe { f (Some (downcast (context))) } } }
}

macro_rules! with_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_context in module {}", module_path!());
    };
}

mkfn!{
    with_context_introspect!();
    # [doc = " Allows access to the current `ImplicitCtxt`."] # [doc = " Panics if there is no `ImplicitCtxt` available."] # [inline] pub fn with_context < F , R > (f : F) -> R where F : for < 'a , 'tcx > FnOnce (& ImplicitCtxt < 'a , 'tcx >) -> R , { with_context_opt (| opt_context | f (opt_context . expect ("no ImplicitCtxt stored in tls"))) }
}

macro_rules! with_related_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_related_context in module {}", module_path!());
    };
}

mkfn!{
    with_related_context_introspect!();
    # [doc = " Allows access to the current `ImplicitCtxt` whose tcx field is the same as the tcx argument"] # [doc = " passed in. This means the closure is given an `ImplicitCtxt` with the same `'tcx` lifetime"] # [doc = " as the `TyCtxt` passed in."] # [doc = " This will panic if you pass it a `TyCtxt` which is different from the current"] # [doc = " `ImplicitCtxt`'s `tcx` field."] # [inline] pub fn with_related_context < 'tcx , F , R > (tcx : TyCtxt < 'tcx > , f : F) -> R where F : FnOnce (& ImplicitCtxt < '_ , 'tcx >) -> R , { with_context (| context | { assert ! (ptr :: eq (context . tcx . gcx as * const _ as * const () , tcx . gcx as * const _ as * const ())) ; let context : & ImplicitCtxt < '_ , '_ > = unsafe { mem :: transmute (context) } ; f (context) }) }
}

macro_rules! with_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with in module {}", module_path!());
    };
}

mkfn!{
    with_introspect!();
    # [doc = " Allows access to the `TyCtxt` in the current `ImplicitCtxt`."] # [doc = " Panics if there is no `ImplicitCtxt` available."] # [inline] pub fn with < F , R > (f : F) -> R where F : for < 'tcx > FnOnce (TyCtxt < 'tcx >) -> R , { with_context (| context | f (context . tcx)) }
}

macro_rules! with_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_opt in module {}", module_path!());
    };
}

mkfn!{
    with_opt_introspect!();
    # [doc = " Allows access to the `TyCtxt` in the current `ImplicitCtxt`."] # [doc = " The closure is passed None if there is no `ImplicitCtxt` available."] # [inline] # [track_caller] pub fn with_opt < F , R > (f : F) -> R where F : for < 'tcx > FnOnce (Option < TyCtxt < 'tcx > >) -> R , { with_context_opt (# [track_caller] | opt_context | f (opt_context . map (| context | context . tcx)) ,) }
}