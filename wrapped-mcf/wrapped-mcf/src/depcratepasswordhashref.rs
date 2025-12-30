// Generated macro for PasswordHashRef (struct)
macro_rules! DepcratePasswordHashRef {
() => {
// Module: crate
// Provides: {"PasswordHashRef"}
// Dependencies: {}
# [doc = " Password hash reference type for hashes encoded in the Modular Crypt Format (MCF),"] # [doc = " e.g. `$<id>$...`."] # [doc = ""] # [doc = " For more information, see [`PasswordHash`]."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord)] pub struct PasswordHashRef < 'a > (& 'a str) ;
};
}
