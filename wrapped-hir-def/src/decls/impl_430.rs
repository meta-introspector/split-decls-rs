macro_rules! deps {
    () => {
        AssocSearchMode!();
        IsTraitAssocItem!();
        SearchMode!();
        Query!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl Query { pub fn new (query : String) -> Self { let lowercased = query . to_lowercase () ; Self { query , lowercased , search_mode : SearchMode :: Exact , assoc_mode : AssocSearchMode :: Include , case_sensitive : false , } } # [doc = " Fuzzy finds items instead of exact matching."] pub fn fuzzy (self) -> Self { Self { search_mode : SearchMode :: Fuzzy , .. self } } pub fn prefix (self) -> Self { Self { search_mode : SearchMode :: Prefix , .. self } } pub fn exact (self) -> Self { Self { search_mode : SearchMode :: Exact , .. self } } # [doc = " Specifies whether we want to include associated items in the result."] pub fn assoc_search_mode (self , assoc_mode : AssocSearchMode) -> Self { Self { assoc_mode , .. self } } # [doc = " Respect casing of the query string when matching."] pub fn case_sensitive (self) -> Self { Self { case_sensitive : true , .. self } } fn matches_assoc_mode (& self , is_trait_assoc_item : IsTraitAssocItem) -> bool { ! matches ! ((is_trait_assoc_item , self . assoc_mode) , (IsTraitAssocItem :: Yes , AssocSearchMode :: Exclude) | (IsTraitAssocItem :: No , AssocSearchMode :: AssocItemsOnly)) } }
    };
}

impl_430!()