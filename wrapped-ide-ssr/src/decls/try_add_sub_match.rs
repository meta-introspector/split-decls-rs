macro_rules! deps {
    () => {
        MatchCollector!();
        Match!();
    };
}

macro_rules! try_add_sub_match {
    () => {
        deps!();
        # [doc = " Attempts to add `m` as a sub-match of `existing`."] fn try_add_sub_match (m : Match , existing : & mut Match , sema : & hir :: Semantics < '_ , ide_db :: RootDatabase > ,) { for p in existing . placeholder_values . values_mut () { if p . range . range . contains_range (m . range . range) { let mut collector = MatchCollector :: default () ; for m in std :: mem :: take (& mut p . inner_matches . matches) { collector . matches_by_node . insert (m . matched_node . clone () , m) ; } collector . add_match (m , sema) ; p . inner_matches = collector . into () ; break ; } } }
    };
}

try_add_sub_match!();