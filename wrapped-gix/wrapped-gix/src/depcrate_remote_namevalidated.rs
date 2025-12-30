// Generated macro for validated (function)
macro_rules! Depcrate_remote_namevalidated {
() => {
// Module: crate::remote::name
// Provides: {"validated"}
// Dependencies: {}
# [doc = " Return `name` if it is valid as symbolic remote name."] # [doc = ""] # [doc = " This means it has to be valid within a the ref path of a tracking branch."] pub fn validated (name : impl Into < BString >) -> Result < BString , Error > { let name = name . into () ; match gix_refspec :: parse (format ! ("refs/heads/test:refs/remotes/{name}/test") . as_str () . into () , gix_refspec :: parse :: Operation :: Fetch ,) { Ok (_) => Ok (name) , Err (err) => Err (Error { source : err , name }) , } }
};
}
