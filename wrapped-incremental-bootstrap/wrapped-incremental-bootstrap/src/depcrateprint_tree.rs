// Generated macro for print_tree (function)
macro_rules! Depcrateprint_tree {
() => {
// Module: crate
// Provides: {"print_tree"}
// Dependencies: {}
fn print_tree (node_id : & str , tree : & HashMap < String , Vec < String > > , cache : & DepCache , depth : usize , visited : & mut HashSet < String >) -> Result < () , Box < dyn std :: error :: Error > > { let indent = "  " . repeat (depth) ; let node = cache . nodes . get (node_id) . unwrap () ; let path_short = node . path . file_name () . and_then (| n | n . to_str ()) . unwrap_or ("unknown") ; println ! ("{}├─ {} ({:x})" , indent , path_short , node . content_hash) ; println ! ("{}│  tokens: {:?}" , indent , node . tokens . iter () . take (3) . collect ::< Vec < _ >> ()) ; if visited . contains (node_id) { println ! ("{}│  (already visited)" , indent) ; return Ok (()) ; } visited . insert (node_id . to_string ()) ; if let Some (deps) = tree . get (node_id) { for (i , dep_id) in deps . iter () . enumerate () { if i < 3 { print_tree (dep_id , tree , cache , depth + 1 , visited) ? ; } else if i == 3 { println ! ("{}├─ ... and {} more dependencies" , "  " . repeat (depth + 1) , deps . len () - 3) ; break ; } } } Ok (()) }
};
}
