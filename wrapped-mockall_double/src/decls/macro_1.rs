macro_rules! macro_1 {
    () => {
        cfg_if ! { if # [cfg (all (feature = "nightly" , not (test)))] { fn compile_error (span : Span , msg : &'static str) { span . unstable () . error (msg) . emit () ; } } else { fn compile_error (_span : Span , msg : & str) { panic ! ("{msg}.  More information may be available when mockall_double is built with the \"nightly\" feature.") ; } } }
    };
}

macro_1!();