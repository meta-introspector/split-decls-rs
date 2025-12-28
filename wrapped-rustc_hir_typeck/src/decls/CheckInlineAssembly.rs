macro_rules! deps {
    () => {
        ItemKind!();
    };
}

macro_rules! CheckInlineAssembly {
    () => {
        deps!();
        struct CheckInlineAssembly { items : Vec < (ItemKind , Span) > , }
    };
}

CheckInlineAssembly!()