// Generated macro for create_header (function)
macro_rules! Depcratecreate_header {
() => {
// Module: crate
// Provides: {"create_header"}
// Dependencies: {}
const fn create_header (which : u8 , n : u64) -> [u8 ; 9] { let bytes = n . to_be_bytes () ; [which , bytes [0] , bytes [1] , bytes [2] , bytes [3] , bytes [4] , bytes [5] , bytes [6] , bytes [7]] }
};
}
