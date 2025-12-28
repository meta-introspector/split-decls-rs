macro_rules! ImplCallVisitor {
    () => {
        struct ImplCallVisitor { calls : std :: collections :: HashMap < String , std :: collections :: HashSet < String > > , }
    };
}

ImplCallVisitor!()