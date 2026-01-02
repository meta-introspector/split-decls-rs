mkitem!{extern crate test ;}
mkuse!{# [cfg (feature = "std")] use backtrace :: Backtrace ;}

macro_rules! trace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace in module {}", module_path!());
    };
}

mkfn!{
    trace_introspect!();
    # [bench] # [cfg (feature = "std")] fn trace (b : & mut test :: Bencher) { # [inline (never)] fn the_function () { backtrace :: trace (| frame | { let ip = frame . ip () ; test :: black_box (ip) ; true }) ; } b . iter (the_function) ; }
}

macro_rules! trace_and_resolve_callback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace_and_resolve_callback in module {}", module_path!());
    };
}

mkfn!{
    trace_and_resolve_callback_introspect!();
    # [bench] # [cfg (feature = "std")] fn trace_and_resolve_callback (b : & mut test :: Bencher) { # [inline (never)] fn the_function () { backtrace :: trace (| frame | { backtrace :: resolve (frame . ip () , | symbol | { let addr = symbol . addr () ; test :: black_box (addr) ; }) ; true }) ; } b . iter (the_function) ; }
}

macro_rules! trace_and_resolve_separate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace_and_resolve_separate in module {}", module_path!());
    };
}

mkfn!{
    trace_and_resolve_separate_introspect!();
    # [bench] # [cfg (feature = "std")] fn trace_and_resolve_separate (b : & mut test :: Bencher) { # [inline (never)] fn the_function (frames : & mut Vec < * mut std :: ffi :: c_void >) { backtrace :: trace (| frame | { frames . push (frame . ip ()) ; true }) ; frames . iter () . for_each (| frame_ip | { backtrace :: resolve (* frame_ip , | symbol | { test :: black_box (symbol) ; }) ; }) ; } let mut frames = Vec :: with_capacity (1024) ; b . iter (| | { the_function (& mut frames) ; frames . clear () ; }) ; }
}

macro_rules! new_unresolved_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_unresolved in module {}", module_path!());
    };
}

mkfn!{
    new_unresolved_introspect!();
    # [bench] # [cfg (feature = "std")] fn new_unresolved (b : & mut test :: Bencher) { # [inline (never)] fn the_function () { let bt = Backtrace :: new_unresolved () ; test :: black_box (bt) ; } b . iter (the_function) ; }
}

macro_rules! new_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new in module {}", module_path!());
    };
}

mkfn!{
    new_introspect!();
    # [bench] # [cfg (feature = "std")] fn new (b : & mut test :: Bencher) { # [inline (never)] fn the_function () { let bt = Backtrace :: new () ; test :: black_box (bt) ; } b . iter (the_function) ; }
}

macro_rules! new_unresolved_and_resolve_separate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_unresolved_and_resolve_separate in module {}", module_path!());
    };
}

mkfn!{
    new_unresolved_and_resolve_separate_introspect!();
    # [bench] # [cfg (feature = "std")] fn new_unresolved_and_resolve_separate (b : & mut test :: Bencher) { # [inline (never)] fn the_function () { let mut bt = Backtrace :: new_unresolved () ; bt . resolve () ; test :: black_box (bt) ; } b . iter (the_function) ; }
}