macro_rules! deps {
    () => {
        LateContext!();
        LifetimeInfoCollector!();
    };
}

macro_rules! check_fn_like {
    () => {
        deps!();
        fn check_fn_like < 'tcx > (cx : & LateContext < 'tcx > , fd : & 'tcx hir :: FnDecl < 'tcx >) { let mut input_map = Default :: default () ; let mut output_map = Default :: default () ; for input in fd . inputs { LifetimeInfoCollector :: collect (input , & mut input_map) ; } if let hir :: FnRetTy :: Return (output) = fd . output { LifetimeInfoCollector :: collect (output , & mut output_map) ; } report_mismatches (cx , & input_map , & output_map) ; }
    };
}

check_fn_like!()