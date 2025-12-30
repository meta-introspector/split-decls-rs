// Generated macro for search_same (function)
macro_rules! Depcratesearch_same {
() => {
// Module: crate
// Provides: {"search_same"}
// Dependencies: {}
# [doc = " Returns a list of groups where elements in each group are equal according to `eq`"] # [doc = ""] # [doc = " - Within each group the elements are sorted by the order they appear in `exprs`"] # [doc = " - The groups themselves are sorted by their first element's appearence in `exprs`"] # [doc = ""] # [doc = " Given functions `eq` and `hash` such that `eq(a, b) == true`"] # [doc = " implies `hash(a) == hash(b)`"] pub fn search_same < T , Hash , Eq > (exprs : & [T] , mut hash : Hash , mut eq : Eq) -> Vec < Vec < & T > > where Hash : FnMut (& T) -> u64 , Eq : FnMut (& T , & T) -> bool , { match exprs { [a , b] if eq (a , b) => return vec ! [vec ! [a , b]] , _ if exprs . len () <= 2 => return vec ! [] , _ => { } , } let mut buckets : UnindexMap < u64 , Vec < Vec < & T > > > = UnindexMap :: default () ; for expr in exprs { match buckets . entry (hash (expr)) { indexmap :: map :: Entry :: Occupied (mut o) => { let bucket = o . get_mut () ; match bucket . iter_mut () . find (| group | eq (expr , group [0])) { Some (group) => group . push (expr) , None => bucket . push (vec ! [expr]) , } } , indexmap :: map :: Entry :: Vacant (v) => { v . insert (vec ! [vec ! [expr]]) ; } , } } buckets . into_values () . flatten () . filter (| group | group . len () > 1) . collect () }
};
}
