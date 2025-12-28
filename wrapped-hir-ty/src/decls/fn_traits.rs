macro_rules! fn_traits {
    () => {
        fn fn_traits (db : & dyn DefDatabase , trait_ : TraitId) -> impl Iterator < Item = TraitId > + '_ { let krate = trait_ . lookup (db) . container . krate () ; utils :: fn_traits (db , krate) }
    };
}

fn_traits!();