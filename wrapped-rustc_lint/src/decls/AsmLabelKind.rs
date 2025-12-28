macro_rules! AsmLabelKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum AsmLabelKind { Named , FormatArg , Binary , }
    };
}

AsmLabelKind!();