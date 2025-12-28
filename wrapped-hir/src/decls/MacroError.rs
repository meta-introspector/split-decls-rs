macro_rules! MacroError {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroError { pub node : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub message : String , pub error : bool , pub kind : & 'static str , }
    };
}

MacroError!();