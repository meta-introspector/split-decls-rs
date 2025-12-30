// Generated macro for tests (module)
macro_rules! Depcrate_compiletests {
() => {
// Module: crate::compile
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ByteClassSet ; # [test] fn byte_classes () { let mut set = ByteClassSet :: new () ; set . set_range (b'a' , b'z') ; let classes = set . byte_classes () ; assert_eq ! (classes [0] , 0) ; assert_eq ! (classes [1] , 0) ; assert_eq ! (classes [2] , 0) ; assert_eq ! (classes [b'a' as usize - 1] , 0) ; assert_eq ! (classes [b'a' as usize] , 1) ; assert_eq ! (classes [b'm' as usize] , 1) ; assert_eq ! (classes [b'z' as usize] , 1) ; assert_eq ! (classes [b'z' as usize + 1] , 2) ; assert_eq ! (classes [254] , 2) ; assert_eq ! (classes [255] , 2) ; let mut set = ByteClassSet :: new () ; set . set_range (0 , 2) ; set . set_range (4 , 6) ; let classes = set . byte_classes () ; assert_eq ! (classes [0] , 0) ; assert_eq ! (classes [1] , 0) ; assert_eq ! (classes [2] , 0) ; assert_eq ! (classes [3] , 1) ; assert_eq ! (classes [4] , 2) ; assert_eq ! (classes [5] , 2) ; assert_eq ! (classes [6] , 2) ; assert_eq ! (classes [7] , 3) ; assert_eq ! (classes [255] , 3) ; } }
};
}
