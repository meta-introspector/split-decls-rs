macro_rules! MetaDirectiveInvocation {
    () => {
        # [doc = " actual directive invocation on SDL definitions"] # [derive (Debug , Clone)] pub struct MetaDirectiveInvocation { # [doc = " name of directive to invoke"] pub name : String , # [doc = " actual arguments passed to directive"] pub args : IndexMap < String , Value > , }
    };
}

MetaDirectiveInvocation!();