// Generated macro for try_split_at (function)
macro_rules! Depcrate_linuxtry_split_at {
() => {
// Module: crate::linux
// Provides: {"try_split_at"}
// Dependencies: {}
fn try_split_at < 'a > (data : & mut & 'a [u8] , index : usize) -> Option < & 'a [u8] > { if data . len () < index { None } else { let (head , tail) = data . split_at (index) ; * data = tail ; Some (head) } }
};
}
