macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! is_name_ref_in_test {
    () => {
        deps!();
        fn is_name_ref_in_test (sema : & Semantics < '_ , RootDatabase > , name_ref : & ast :: NameRef) -> bool { name_ref . syntax () . ancestors () . any (| node | match ast :: Fn :: cast (node) { Some (it) => sema . to_def (& it) . is_some_and (| func | func . is_test (sema . db)) , None => false , }) }
    };
}

is_name_ref_in_test!();