macro_rules! UnresolvedMacroCall {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct UnresolvedMacroCall { pub macro_call : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub path : ModPath , pub is_bang : bool , }
    };
}

UnresolvedMacroCall!()