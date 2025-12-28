macro_rules! ExpandErrorKind {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Hash)] pub enum ExpandErrorKind { # [doc = " Attribute macro expansion is disabled."] ProcMacroAttrExpansionDisabled , MissingProcMacroExpander (Crate) , # [doc = " The macro for this call is disabled."] MacroDisabled , # [doc = " The macro definition has errors."] MacroDefinition , Mbe (mbe :: ExpandErrorKind) , RecursionOverflow , Other (Box < str >) , ProcMacroPanic (Box < str >) , }
    };
}

ExpandErrorKind!();