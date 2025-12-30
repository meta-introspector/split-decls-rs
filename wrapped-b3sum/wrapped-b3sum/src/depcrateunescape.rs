// Generated macro for unescape (function)
macro_rules! Depcrateunescape {
() => {
// Module: crate
// Provides: {"unescape"}
// Dependencies: {}
fn unescape (mut path : & str) -> anyhow :: Result < String > { let mut unescaped = String :: with_capacity (2 * path . len ()) ; while let Some (i) = path . find ('\\') { ensure ! (i < path . len () - 1 , "Invalid backslash escape") ; unescaped . push_str (& path [.. i]) ; match path [i + 1 ..] . chars () . next () . unwrap () { 'n' => unescaped . push_str ("\n") , 'r' => unescaped . push_str ("\r") , '\\' => unescaped . push_str ("\\") , _ => bail ! ("Invalid backslash escape") , } path = & path [i + 2 ..] ; } unescaped . push_str (path) ; Ok (unescaped) }
};
}
