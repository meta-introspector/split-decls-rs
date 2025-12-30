// Generated macro for tests (module)
macro_rules! Depcrate_idtests {
() => {
// Module: crate::id
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_oid () { let actual = std :: mem :: size_of :: < Id < '_ > > () ; let ceiling = 32 ; assert ! (actual <= ceiling , "size of oid shouldn't change without notice: {actual} <= {ceiling}") ; } }
};
}
