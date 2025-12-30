// Generated macro for test2 (module)
macro_rules! Depcratetest2 {
() => {
// Module: crate
// Provides: {"test2"}
// Dependencies: {}
pub mod test2 { use maplit :: hashset ; use std :: collections :: HashSet ; pub fn make_map () -> HashSet < String > { let s = String :: from ; hashset ! { s ("a1") , s ("a2") , } } pub fn convert () -> HashSet < String > { maplit :: convert_args ! (hashset ! ("a" , "b")) } }
};
}
