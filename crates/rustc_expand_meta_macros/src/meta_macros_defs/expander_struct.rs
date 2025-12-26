#[macro_export]
macro_rules! ExpanderStruct {
    () => {
        pub struct MacroExpander<'a, 'b, DRT: OpaqueDeriveResolution + 'static> {
            // Lifetimes and DRT are implicitly captured or inferred from the usage context
            // of CtxSpecial! and the surrounding impl block.
            cx: CtxSpecial!('a, 'b, DRT),
            monotonic: bool,
        }
    };
}