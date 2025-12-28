macro_rules! SearchMode {
    () => {
        # [doc = " A way to match import map contents against the search query."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum SearchMode { # [doc = " Import map entry should strictly match the query string."] Exact , # [doc = " Import map entry should contain all letters from the query string,"] # [doc = " in the same order, but not necessary adjacent."] Fuzzy , # [doc = " Import map entry should match the query string by prefix."] Prefix , }
    };
}

SearchMode!();