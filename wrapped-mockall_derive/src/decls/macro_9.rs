macro_rules! macro_9 {
    () => {
        cfg_if ! { if # [cfg (all (feature = "nightly_derive" , not (test)))] { fn compile_error (span : Span , msg : & str) { span . unstable () . error (msg) . emit () ; } } else { fn compile_error (_span : Span , msg : & str) { panic ! ("{msg}.  More information may be available when mockall is built with the \"nightly\" feature.") ; } } }
    };
}

macro_9!()