macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! indexmap {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [macro_export] # [doc = " Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::indexmap;"] # [doc = ""] # [doc = " let map = indexmap!{"] # [doc = "     \"a\" => 1,"] # [doc = "     \"b\" => 2,"] # [doc = " };"] # [doc = " assert_eq!(map[\"a\"], 1);"] # [doc = " assert_eq!(map[\"b\"], 2);"] # [doc = " assert_eq!(map.get(\"c\"), None);"] # [doc = ""] # [doc = " // \"a\" is the first key"] # [doc = " assert_eq!(map.keys().next(), Some(&\"a\"));"] # [doc = " ```"] macro_rules ! indexmap { ($ ($ key : expr => $ value : expr ,) +) => { $ crate :: indexmap ! ($ ($ key => $ value) ,+) } ; ($ ($ key : expr => $ value : expr) ,*) => { { const CAP : usize = < [()] >:: len (& [$ ({ stringify ! ($ key) ; }) ,*]) ; let mut map = $ crate :: IndexMap :: with_capacity (CAP) ; $ (map . insert ($ key , $ value) ;) * map } } ; }
    };
}

indexmap!();