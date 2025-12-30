// Generated macro for function (module)
macro_rules! Depcrate_repository_catfunction {
() => {
// Module: crate::repository::cat
// Provides: {"function"}
// Dependencies: {}
pub (super) mod function { use crate :: repository :: revision :: resolve :: TreeMode ; pub fn cat (repo : gix :: Repository , revspec : & str , out : impl std :: io :: Write) -> anyhow :: Result < () > { super :: display_object (& repo , repo . rev_parse (revspec) ? , TreeMode :: Pretty , None , out) ? ; Ok (()) } }
};
}
