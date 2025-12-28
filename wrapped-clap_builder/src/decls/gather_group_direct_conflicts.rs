macro_rules! deps {
    () => {
        Id!();
        ArgGroup!();
    };
}

macro_rules! gather_group_direct_conflicts {
    () => {
        deps!();
        fn gather_group_direct_conflicts (group : & ArgGroup) -> Vec < Id > { group . conflicts . clone () }
    };
}

gather_group_direct_conflicts!();