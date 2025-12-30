// Generated macro for get_remaining (function)
macro_rules! Depcrate_witget_remaining {
() => {
// Module: crate::wit
// Provides: {"get_remaining"}
// Dependencies: {}
fn get_remaining < 'a > (data : & mut & 'a [u8]) -> Option < & 'a [u8] > { if data . is_empty () { return None ; } let len = u32 :: from_le_bytes ([data [0] , data [1] , data [2] , data [3]]) as usize ; let (a , b) = data [4 ..] . split_at (len) ; * data = b ; Some (a) }
};
}
