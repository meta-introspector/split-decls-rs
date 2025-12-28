macro_rules! deps {
    () => {
        Command!();
        ArgPredicate!();
        Conflicts!();
        Id!();
        FlatMap!();
        ArgMatcher!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl Conflicts { fn with_args (cmd : & Command , matcher : & ArgMatcher) -> Self { let mut potential = FlatMap :: new () ; potential . extend_unchecked (matcher . args () . filter (| (_ , matched) | matched . check_explicit (& ArgPredicate :: IsPresent)) . map (| (id , _) | { let conf = gather_direct_conflicts (cmd , id) ; (id . clone () , conf) }) ,) ; Self { potential } } fn gather_conflicts (& self , cmd : & Command , arg_id : & Id) -> Vec < Id > { debug ! ("Conflicts::gather_conflicts: arg={arg_id:?}") ; let mut conflicts = Vec :: new () ; let arg_id_conflicts_storage ; let arg_id_conflicts = if let Some (arg_id_conflicts) = self . get_direct_conflicts (arg_id) { arg_id_conflicts } else { arg_id_conflicts_storage = gather_direct_conflicts (cmd , arg_id) ; & arg_id_conflicts_storage } ; for (other_arg_id , other_arg_id_conflicts) in self . potential . iter () { if arg_id == other_arg_id { continue ; } if arg_id_conflicts . contains (other_arg_id) { conflicts . push (other_arg_id . clone ()) ; } if other_arg_id_conflicts . contains (arg_id) { conflicts . push (other_arg_id . clone ()) ; } } debug ! ("Conflicts::gather_conflicts: conflicts={conflicts:?}") ; conflicts } fn get_direct_conflicts (& self , arg_id : & Id) -> Option < & [Id] > { self . potential . get (arg_id) . map (Vec :: as_slice) } }
    };
}

impl_521!()