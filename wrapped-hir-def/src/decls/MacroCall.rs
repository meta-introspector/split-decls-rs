macro_rules! MacroCall {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroCall { # [doc = " Path to the called macro."] pub path : Interned < ModPath > , pub expand_to : ExpandTo , pub ctxt : SyntaxContext , }
    };
}

MacroCall!()