macro_rules! Span {
    () => {
        # [doc = " A region of source code, along with macro expansion information."] # [derive (Copy , Clone)] pub struct Span { inner : imp :: Span , _marker : ProcMacroAutoTraits , }
    };
}

Span!()