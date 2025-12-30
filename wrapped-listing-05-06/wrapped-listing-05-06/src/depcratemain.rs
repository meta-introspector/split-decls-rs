// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let user1 = User { email : String :: from ("someone@example.com") , username : String :: from ("someusername123") , active : true , sign_in_count : 1 , } ; let user2 = User { active : user1 . active , username : user1 . username , email : String :: from ("another@example.com") , sign_in_count : user1 . sign_in_count , } ; }
};
}
