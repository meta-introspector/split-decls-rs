macro_rules! QualifierCtx {
    () => {
        # [doc = " Existing qualifiers for the thing we are currently completing."] # [derive (Debug , Default)] pub (crate) struct QualifierCtx { pub (crate) async_tok : Option < SyntaxToken > , pub (crate) unsafe_tok : Option < SyntaxToken > , pub (crate) safe_tok : Option < SyntaxToken > , pub (crate) vis_node : Option < ast :: Visibility > , }
    };
}

QualifierCtx!();