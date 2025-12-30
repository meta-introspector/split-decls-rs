// Generated macro for Reference (enum)
macro_rules! Depcrate_readReference {
() => {
// Module: crate::read
// Provides: {"Reference"}
// Dependencies: {}
pub enum Reference < 'b , 'c , T > where T : ? Sized + 'static , { Borrowed (& 'b T) , Copied (& 'c T) , }
};
}
