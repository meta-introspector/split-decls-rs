macro_rules! deps {
    () => {
        ImportMap!();
        Union!();
        Query!();
        Complete!();
        ItemInNs!();
        DefDatabase!();
    };
}

macro_rules! search_maps {
    () => {
        deps!();
        fn search_maps (_db : & dyn DefDatabase , import_maps : & [Arc < ImportMap >] , mut stream : fst :: map :: Union < '_ > , query : & Query ,) -> FxHashSet < (ItemInNs , Complete) > { let mut res = FxHashSet :: default () ; while let Some ((_ , indexed_values)) = stream . next () { for & IndexedValue { index : import_map_idx , value } in indexed_values { let end = (value & 0xFFFF_FFFF) as usize ; let start = (value >> 32) as usize ; let ImportMap { item_to_info_map , importables , .. } = & * import_maps [import_map_idx] ; let importables = & importables [start .. end] ; let iter = importables . iter () . copied () . filter_map (| (item , info_idx) | { let (import_infos , assoc_mode) = & item_to_info_map [& item] ; query . matches_assoc_mode (* assoc_mode) . then (| | (item , & import_infos [info_idx as usize])) }) . filter (| & (_ , info) | { query . search_mode . check (& query . query , query . case_sensitive , info . name . as_str ()) }) . map (| (item , import_info) | (item , import_info . complete)) ; res . extend (iter) ; } } res }
    };
}

search_maps!();