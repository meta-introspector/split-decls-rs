macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! EarlyContextAndPass {
    () => {
        deps!();
        # [doc = " Implements the AST traversal for early lint passes. `T` provides the"] # [doc = " `check_*` methods."] pub struct EarlyContextAndPass < 'ecx , 'tcx , T : EarlyLintPass > { context : EarlyContext < 'ecx > , tcx : Option < TyCtxt < 'tcx > > , pass : T , }
    };
}

EarlyContextAndPass!();