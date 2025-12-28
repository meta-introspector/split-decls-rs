macro_rules! Query {
    () => {
        # [derive (Debug , Clone)] pub struct Query { query : String , lowercased : String , mode : SearchMode , assoc_mode : AssocSearchMode , case_sensitive : bool , only_types : bool , libs : bool , exclude_imports : bool , }
    };
}

Query!();