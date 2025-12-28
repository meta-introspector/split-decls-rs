macro_rules! deps {
    () => {
        MetaVisibleFn!();
        MetaInputValue!();
    };
}

macro_rules! MetaDirective {
    () => {
        deps!();
        pub struct MetaDirective { pub name : String , pub description : Option < String > , pub locations : Vec < __DirectiveLocation > , pub args : IndexMap < String , MetaInputValue > , pub is_repeatable : bool , pub visible : Option < MetaVisibleFn > , pub composable : Option < String > , }
    };
}

MetaDirective!()