macro_rules! deps {
    () => {
        SearchMode!();
        AssocSearchMode!();
    };
}

macro_rules! Query {
    () => {
        deps!();
        # [derive (Debug)] pub struct Query { query : String , lowercased : String , search_mode : SearchMode , assoc_mode : AssocSearchMode , case_sensitive : bool , }
    };
}

Query!();