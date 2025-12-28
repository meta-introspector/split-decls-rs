macro_rules! deps {
    () => {
        SemanticsImpl!();
    };
}

macro_rules! ToDef {
    () => {
        deps!();
        pub trait ToDef : AstNode + Clone { type Def ; fn to_def (sema : & SemanticsImpl < '_ > , src : InFile < & Self >) -> Option < Self :: Def > ; }
    };
}

ToDef!();