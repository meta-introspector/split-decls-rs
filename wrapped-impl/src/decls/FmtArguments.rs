macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! FmtArguments {
    () => {
        deps!();
        struct FmtArguments { named : BTreeSet < IdentUnraw > , first_unnamed : Option < TokenStream > , }
    };
}

FmtArguments!();