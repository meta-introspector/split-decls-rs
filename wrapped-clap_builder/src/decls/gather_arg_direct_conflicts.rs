macro_rules! deps {
    () => {
        Arg!();
        Id!();
        Command!();
    };
}

macro_rules! gather_arg_direct_conflicts {
    () => {
        deps!();
        fn gather_arg_direct_conflicts (cmd : & Command , arg : & Arg) -> Vec < Id > { let mut conf = arg . blacklist . clone () ; for group_id in cmd . groups_for_arg (arg . get_id ()) { let group = cmd . find_group (& group_id) . expect (INTERNAL_ERROR_MSG) ; conf . extend (group . conflicts . iter () . cloned ()) ; if ! group . multiple { for member_id in & group . args { if member_id != arg . get_id () { conf . push (member_id . clone ()) ; } } } } conf . extend (arg . overrides . iter () . cloned ()) ; conf }
    };
}

gather_arg_direct_conflicts!()