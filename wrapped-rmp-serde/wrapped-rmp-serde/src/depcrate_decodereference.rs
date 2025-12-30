// Generated macro for Reference (enum)
macro_rules! Depcrate_decodeReference {
() => {
// Module: crate::decode
// Provides: {"Reference"}
// Dependencies: {}
# [doc = " Unification of both borrowed and non-borrowed reference types."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Reference < 'b , 'c , T : ? Sized + 'static > { # [doc = " The reference is pointed at data that was borrowed."] Borrowed (& 'b T) , # [doc = " The reference is pointed at data that was copied."] Copied (& 'c T) , }
};
}
