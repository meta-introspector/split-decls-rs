// Generated macro for trim_cow (function)
macro_rules! Depcrate_eventstrim_cow {
() => {
// Module: crate::events
// Provides: {"trim_cow"}
// Dependencies: {}
fn trim_cow < 'a , F > (value : Cow < 'a , [u8] > , trim : F) -> Cow < 'a , [u8] > where F : FnOnce (& [u8]) -> & [u8] , { match value { Cow :: Borrowed (bytes) => Cow :: Borrowed (trim (bytes)) , Cow :: Owned (mut bytes) => { let trimmed = trim (& bytes) ; if trimmed . len () != bytes . len () { bytes = trimmed . to_vec () ; } Cow :: Owned (bytes) } } }
};
}
