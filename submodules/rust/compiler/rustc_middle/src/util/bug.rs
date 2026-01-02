mkuse!{use std :: fmt ;}
mkuse!{use std :: panic :: { Location , panic_any } ;}
mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: ty :: { TyCtxt , tls } ;}

macro_rules! bug_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bug_fmt in module {}", module_path!());
    };
}

mkfn!{
    bug_fmt_introspect!();
    # [cold] # [inline (never)] # [track_caller] pub fn bug_fmt (args : fmt :: Arguments < '_ >) -> ! { opt_span_bug_fmt (None :: < Span > , args , Location :: caller ()) ; }
}

macro_rules! span_bug_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function span_bug_fmt in module {}", module_path!());
    };
}

mkfn!{
    span_bug_fmt_introspect!();
    # [cold] # [inline (never)] # [track_caller] pub fn span_bug_fmt < S : Into < MultiSpan > > (span : S , args : fmt :: Arguments < '_ >) -> ! { opt_span_bug_fmt (Some (span) , args , Location :: caller ()) ; }
}

macro_rules! opt_span_bug_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opt_span_bug_fmt in module {}", module_path!());
    };
}

mkfn!{
    opt_span_bug_fmt_introspect!();
    # [track_caller] fn opt_span_bug_fmt < S : Into < MultiSpan > > (span : Option < S > , args : fmt :: Arguments < '_ > , location : & Location < '_ > ,) -> ! { tls :: with_opt (# [track_caller] move | tcx | { let msg = format ! ("{location}: {args}") ; match (tcx , span) { (Some (tcx) , Some (span)) => tcx . dcx () . span_bug (span , msg) , (Some (tcx) , None) => tcx . dcx () . bug (msg) , (None , _) => panic_any (msg) , } } ,) }
}

macro_rules! trigger_delayed_bug_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trigger_delayed_bug in module {}", module_path!());
    };
}

mkfn!{
    trigger_delayed_bug_introspect!();
    # [doc = " A query to trigger a delayed bug. Clearly, if one has a `tcx` one can already trigger a"] # [doc = " delayed bug, so what is the point of this? It exists to help us test the interaction of delayed"] # [doc = " bugs with the query system and incremental."] pub fn trigger_delayed_bug (tcx : TyCtxt < '_ > , key : rustc_hir :: def_id :: DefId) { tcx . dcx () . span_delayed_bug (tcx . def_span (key) , "delayed bug triggered by #[rustc_delayed_bug_from_inside_query]" ,) ; }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut crate :: query :: Providers) { * providers = crate :: query :: Providers { trigger_delayed_bug , .. * providers } ; }
}