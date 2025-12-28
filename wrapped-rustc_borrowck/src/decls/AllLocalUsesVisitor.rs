macro_rules! AllLocalUsesVisitor {
    () => {
        struct AllLocalUsesVisitor { for_local : Local , uses : BTreeSet < Location > , }
    };
}

AllLocalUsesVisitor!();