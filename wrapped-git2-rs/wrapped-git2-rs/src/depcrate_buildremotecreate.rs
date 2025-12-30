// Generated macro for RemoteCreate (type)
macro_rules! Depcrate_buildRemoteCreate {
() => {
// Module: crate::build
// Provides: {"RemoteCreate"}
// Dependencies: {}
# [doc = " Type of callback passed to `RepoBuilder::remote_create`."] # [doc = ""] # [doc = " The second and third arguments are the remote's name and the remote's URL."] pub type RemoteCreate < 'cb > = dyn for < 'a > FnMut (& 'a Repository , & str , & str) -> Result < Remote < 'a > , Error > + 'cb ;
};
}
