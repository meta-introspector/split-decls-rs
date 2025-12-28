macro_rules! deps {
    () => {
        Command!();
        Id!();
    };
}

macro_rules! gather_direct_conflicts {
    () => {
        deps!();
        fn gather_direct_conflicts (cmd : & Command , id : & Id) -> Vec < Id > { let conf = if let Some (arg) = cmd . find (id) { gather_arg_direct_conflicts (cmd , arg) } else if let Some (group) = cmd . find_group (id) { gather_group_direct_conflicts (group) } else { debug_assert ! (false , "id={id:?} is unknown") ; Vec :: new () } ; debug ! ("Conflicts::gather_direct_conflicts id={id:?}, conflicts={conf:?}" ,) ; conf }
    };
}

gather_direct_conflicts!()