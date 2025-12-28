macro_rules! MacroExpansionParseError {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroExpansionParseError { pub node : InFile < SyntaxNodePtr > , pub precise_location : Option < TextRange > , pub errors : Arc < [SyntaxError] > , }
    };
}

MacroExpansionParseError!()