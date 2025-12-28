macro_rules! BreakContextKind {
    () => {
        # [derive (PartialEq)] enum BreakContextKind { Break , Continue , }
    };
}

BreakContextKind!();