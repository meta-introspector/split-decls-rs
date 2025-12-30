// Generated macro for tests (module)
macro_rules! Depcrate_frontend_zerovectests {
() => {
// Module: crate::frontend::zerovec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: SinglePlaceholderPattern ; use litemap :: LiteMap ; use zerovec :: ZeroMap ; # [test] fn test_zeromap () { let pattern = SinglePlaceholderPattern :: try_from_str ("Hello, {0}!" , Default :: default ()) . unwrap () ; let mut litemap = LiteMap :: < u32 , Box < SinglePlaceholderPattern > > :: new_vec () ; litemap . insert (0 , pattern . clone ()) ; let zeromap = ZeroMap :: < u32 , SinglePlaceholderPattern > :: from_iter (litemap) ; let recovered_pattern = zeromap . get (& 0) . unwrap () ; assert_eq ! (&* pattern , recovered_pattern) ; } }
};
}
