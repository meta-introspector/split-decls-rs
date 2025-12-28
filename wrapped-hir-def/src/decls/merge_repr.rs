macro_rules! merge_repr {
    () => {
        fn merge_repr (this : & mut ReprOptions , other : ReprOptions) { let ReprOptions { int , align , pack , flags , field_shuffle_seed : _ } = this ; flags . insert (other . flags) ; * align = (* align) . max (other . align) ; * pack = match (* pack , other . pack) { (Some (pack) , None) | (None , Some (pack)) => Some (pack) , _ => (* pack) . min (other . pack) , } ; if other . int . is_some () { * int = other . int ; } }
    };
}

merge_repr!();