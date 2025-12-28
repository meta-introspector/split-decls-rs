macro_rules! deps {
    () => {
        SpanMap!();
        ExpansionSpanMap!();
        InFile!();
        MacroCallLoc!();
        InMacroFile!();
    };
}

macro_rules! ExpansionInfo {
    () => {
        deps!();
        # [doc = " ExpansionInfo mainly describes how to map text range between src and expanded macro"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ExpansionInfo { expanded : InMacroFile < SyntaxNode > , # [doc = " The argument TokenTree or item for attributes"] arg : InFile < Option < SyntaxNode > > , exp_map : Arc < ExpansionSpanMap > , arg_map : SpanMap , loc : MacroCallLoc , }
    };
}

ExpansionInfo!();