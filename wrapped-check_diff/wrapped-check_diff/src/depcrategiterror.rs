// Generated macro for GitError (enum)
macro_rules! DepcrateGitError {
() => {
// Module: crate
// Provides: {"GitError"}
// Dependencies: {}
# [derive (Debug)] pub enum GitError { FailedClone { stdout : Vec < u8 > , stderr : Vec < u8 > } , FailedRemoteAdd { stdout : Vec < u8 > , stderr : Vec < u8 > } , FailedFetch { stdout : Vec < u8 > , stderr : Vec < u8 > } , FailedSwitch { stdout : Vec < u8 > , stderr : Vec < u8 > } , IO (std :: io :: Error) , }
};
}
