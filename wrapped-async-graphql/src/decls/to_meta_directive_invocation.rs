macro_rules! deps {
    () => {
        MetaDirectiveInvocation!();
        Directive!();
    };
}

macro_rules! to_meta_directive_invocation {
    () => {
        deps!();
        pub fn to_meta_directive_invocation (directives : Vec < Directive >) -> Vec < MetaDirectiveInvocation > { directives . into_iter () . map (MetaDirectiveInvocation :: from) . collect () }
    };
}

to_meta_directive_invocation!();