// Generated macro for find_attr (function)
macro_rules! Depcrate_generatefind_attr {
() => {
// Module: crate::generate
// Provides: {"find_attr"}
// Dependencies: {}
fn find_attr < 'a > (a : & 'a Vec < xml :: attribute :: OwnedAttribute > , n : & str) -> Result < & 'a str , Box < dyn error :: Error > > { a . into_iter () . find (| q | q . name . prefix . is_none () && q . name . local_name == n) . map (| f | & * f . value) . ok_or_else (| | format ! ("attribute not found: {:?}" , n) . into ()) }
};
}
