macro_rules! deps {
    () => {
        DefDatabase!();
        Complete!();
        SearchMode!();
        Query!();
        ItemInNs!();
    };
}

macro_rules! search_dependencies {
    () => {
        deps!();
        # [doc = " Searches dependencies of `krate` for an importable name matching `query`."] # [doc = ""] # [doc = " This returns a list of items that could be imported from dependencies of `krate`."] pub fn search_dependencies (db : & dyn DefDatabase , krate : Crate , query : & Query ,) -> FxHashSet < (ItemInNs , Complete) > { let _p = tracing :: info_span ! ("search_dependencies" , ? query) . entered () ; let import_maps : Vec < _ > = krate . data (db) . dependencies . iter () . map (| dep | db . import_map (dep . crate_id)) . collect () ; let mut op = fst :: map :: OpBuilder :: new () ; match query . search_mode { SearchMode :: Exact => { let automaton = fst :: automaton :: Str :: new (& query . lowercased) ; for map in & import_maps { op = op . add (map . fst . search (& automaton)) ; } search_maps (db , & import_maps , op . union () , query) } SearchMode :: Fuzzy => { let automaton = fst :: automaton :: Subsequence :: new (& query . lowercased) ; for map in & import_maps { op = op . add (map . fst . search (& automaton)) ; } search_maps (db , & import_maps , op . union () , query) } SearchMode :: Prefix => { let automaton = fst :: automaton :: Str :: new (& query . lowercased) . starts_with () ; for map in & import_maps { op = op . add (map . fst . search (& automaton)) ; } search_maps (db , & import_maps , op . union () , query) } } }
    };
}

search_dependencies!()