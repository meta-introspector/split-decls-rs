// Generated macro for assert_no_lifetimes (function)
macro_rules! Depcrate_parserassert_no_lifetimes {
() => {
// Module: crate::parser
// Provides: {"assert_no_lifetimes"}
// Dependencies: {}
# [doc = " Check there are no lifetimes on the function."] fn assert_no_lifetimes (sig : & syn :: Signature) -> Result < () , Diagnostic > { struct Walk { diagnostics : Vec < Diagnostic > , } impl < 'ast > syn :: visit :: Visit < 'ast > for Walk { fn visit_lifetime (& mut self , i : & 'ast syn :: Lifetime) { self . diagnostics . push (err_span ! (i , "it is currently not sound to use lifetimes in function \
                 signatures")) ; } } let mut walk = Walk { diagnostics : Vec :: new () , } ; syn :: visit :: Visit :: visit_signature (& mut walk , sig) ; Diagnostic :: from_vec (walk . diagnostics) }
};
}
