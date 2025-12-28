macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! indexset {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [macro_export] # [doc = " Create an [`IndexSet`][crate::IndexSet] from a list of values"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::indexset;"] # [doc = ""] # [doc = " let set = indexset!{"] # [doc = "     \"a\","] # [doc = "     \"b\","] # [doc = " };"] # [doc = " assert!(set.contains(\"a\"));"] # [doc = " assert!(set.contains(\"b\"));"] # [doc = " assert!(!set.contains(\"c\"));"] # [doc = ""] # [doc = " // \"a\" is the first value"] # [doc = " assert_eq!(set.iter().next(), Some(&\"a\"));"] # [doc = " ```"] macro_rules ! indexset { ($ ($ value : expr ,) +) => { $ crate :: indexset ! ($ ($ value) ,+) } ; ($ ($ value : expr) ,*) => { { const CAP : usize = < [()] >:: len (& [$ ({ stringify ! ($ value) ; }) ,*]) ; let mut set = $ crate :: IndexSet :: with_capacity (CAP) ; $ (set . insert ($ value) ;) * set } } ; }
    };
}

indexset!();