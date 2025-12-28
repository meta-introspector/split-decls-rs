macro_rules! ExpansionOutcome {
    () => {
        # [derive (Debug)] enum ExpansionOutcome { Same , Different (Vec < u8 > , Vec < u8 >) , Update , ExpandError (Vec < u8 >) , NoExpandedFileFound , }
    };
}

ExpansionOutcome!()