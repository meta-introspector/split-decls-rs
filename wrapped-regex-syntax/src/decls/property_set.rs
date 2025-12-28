macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! property_set {
    () => {
        deps!();
        # [allow (dead_code)] fn property_set (name_map : & 'static [(& 'static str , Range)] , canonical : & 'static str ,) -> Option < Range > { name_map . binary_search_by_key (& canonical , | x | x . 0) . ok () . map (| i | name_map [i] . 1) }
    };
}

property_set!();