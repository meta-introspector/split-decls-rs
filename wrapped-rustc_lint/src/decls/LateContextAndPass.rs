macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! LateContextAndPass {
    () => {
        deps!();
        # [doc = " Implements the AST traversal for late lint passes. `T` provides the"] # [doc = " `check_*` methods."] struct LateContextAndPass < 'tcx , T : LateLintPass < 'tcx > > { context : LateContext < 'tcx > , pass : T , }
    };
}

LateContextAndPass!()