macro_rules! arg_conflicts {
    () => {
        fn arg_conflicts (cmd : & Command , arg : & Arg , app_global : Option < & Command >) -> String { fn push_conflicts (conflicts : & [& Arg] , res : & mut Vec < String >) { for conflict in conflicts { if let Some (s) = conflict . get_short () { res . push (format ! ("-{s}")) ; } if let Some (l) = conflict . get_long () { res . push (format ! ("--{l}")) ; } } } let mut res = vec ! [] ; match (app_global , arg . is_global_set ()) { (Some (x) , true) => { let conflicts = x . get_arg_conflicts_with (arg) ; if conflicts . is_empty () { return String :: new () ; } push_conflicts (& conflicts , & mut res) ; } (_ , _) => { let conflicts = cmd . get_arg_conflicts_with (arg) ; if conflicts . is_empty () { return String :: new () ; } push_conflicts (& conflicts , & mut res) ; } } ; format ! ("({})" , res . join (" ")) }
    };
}

arg_conflicts!()