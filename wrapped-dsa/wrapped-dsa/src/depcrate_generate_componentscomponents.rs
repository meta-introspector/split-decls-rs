// Generated macro for Components (struct)
macro_rules! Depcrate_generate_componentsComponents {
() => {
// Module: crate::generate::components
// Provides: {"Components"}
// Dependencies: {}
# [doc = " The common components of an DSA keypair"] # [doc = ""] # [doc = " (the prime p, quotient q and generator g)"] # [derive (Clone , Debug , PartialEq , PartialOrd)] # [must_use] pub struct Components { # [doc = " Prime p"] p : Odd < BoxedUint > , # [doc = " Quotient q"] q : NonZero < BoxedUint > , # [doc = " Generator g"] g : NonZero < BoxedUint > , pub (crate) key_size : KeySize , }
};
}
