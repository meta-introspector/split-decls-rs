// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let store = Inventory { shirts : vec ! [ShirtColor :: Blue , ShirtColor :: Red , ShirtColor :: Blue] , } ; let user_pref1 = Some (ShirtColor :: Red) ; let giveaway1 = store . giveaway (user_pref1) ; println ! ("The user with preference {:?} gets {:?}" , user_pref1 , giveaway1) ; let user_pref2 = None ; let giveaway2 = store . giveaway (user_pref2) ; println ! ("The user with preference {:?} gets {:?}" , user_pref2 , giveaway2) ; }
};
}
