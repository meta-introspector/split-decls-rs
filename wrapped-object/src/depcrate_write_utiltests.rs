// Generated macro for tests (module)
macro_rules! Depcrate_write_utiltests {
() => {
// Module: crate::write::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn bytes_mut () { let data = vec ! [0x01 , 0x23 , 0x45 , 0x67] ; let mut bytes = data . clone () ; bytes . extend_from_slice (bytes_of (& u16 :: to_be (0x89ab))) ; assert_eq ! (bytes , [0x01 , 0x23 , 0x45 , 0x67 , 0x89 , 0xab]) ; let mut bytes = data . clone () ; assert_eq ! (bytes . write_at (0 , & u16 :: to_be (0x89ab)) , Ok (())) ; assert_eq ! (bytes , [0x89 , 0xab , 0x45 , 0x67]) ; let mut bytes = data . clone () ; assert_eq ! (bytes . write_at (2 , & u16 :: to_be (0x89ab)) , Ok (())) ; assert_eq ! (bytes , [0x01 , 0x23 , 0x89 , 0xab]) ; assert_eq ! (bytes . write_at (3 , & u16 :: to_be (0x89ab)) , Err (())) ; assert_eq ! (bytes . write_at (4 , & u16 :: to_be (0x89ab)) , Err (())) ; assert_eq ! ([] . write_at (0 , & u32 :: to_be (0x89ab)) , Err (())) ; } }
};
}
