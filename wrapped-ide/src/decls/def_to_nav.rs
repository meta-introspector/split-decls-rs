macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! def_to_nav {
    () => {
        deps!();
        fn def_to_nav (sema : & Semantics < '_ , RootDatabase > , def : Definition) -> Vec < NavigationTarget > { def . try_to_nav (sema) . map (| it | it . collect ()) . unwrap_or_default () }
    };
}

def_to_nav!()