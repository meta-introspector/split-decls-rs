// Generated macro for load_table (function)
macro_rules! Depcrateload_table {
() => {
// Module: crate
// Provides: {"load_table"}
// Dependencies: {}
# [doc = " Returns root of tree"] fn load_table () -> (Vec < (usize , String) > , Box < Node >) { let mut lines = TABLE . lines () ; let mut root : Option < Box < Node > > = None ; let mut encode = vec ! [] ; lines . next () ; for (i , line) in lines . enumerate () { let mut bits : Vec < bool > = vec ! [] ; for & b in & line . as_bytes () [12 .. 45] { match b { b'1' => bits . push (true) , b'0' => bits . push (false) , b'|' | b' ' => { } _ => panic ! ("unexpected byte; {:?}" , b) , } } let hex = line [50 .. 59] . trim () ; encode . push ((bits . len () , hex . to_string ())) ; match root { Some (ref mut node) => { node . insert (i , & bits) ; } None => { root = Some (Node :: new (i , & bits)) ; } } } let mut root = root . unwrap () ; let mut id = 0 ; root . set_id (& mut id , & mut vec ! []) ; root . compute_transitions (& root) ; (encode , root) }
};
}
