// Generated macro for impl_proptest_arbitrary (function)
macro_rules! Depcrate_deriveimpl_proptest_arbitrary {
() => {
// Module: crate::derive
// Provides: {"impl_proptest_arbitrary"}
// Dependencies: {}
pub fn impl_proptest_arbitrary (ast : DeriveInput) -> TokenStream { let mut ctx = Context :: default () ; let result = derive_proptest_arbitrary (& mut ctx , ast) ; match (result , ctx . check ()) { (Ok (derive) , Ok (())) => derive , (_ , Err (err)) => err , (Err (result) , Ok (())) => panic ! ("[proptest_derive]: internal error, this is a bug! \
             result: {:?}" , result) , } }
};
}
