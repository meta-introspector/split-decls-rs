macro_rules! BadRtn {
    () => {
        # [derive (Debug)] pub struct BadRtn { pub rtn : InFile < AstPtr < ast :: ReturnTypeSyntax > > , }
    };
}

BadRtn!();