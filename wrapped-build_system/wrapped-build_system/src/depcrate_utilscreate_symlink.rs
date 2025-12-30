// Generated macro for create_symlink (function)
macro_rules! Depcrate_utilscreate_symlink {
() => {
// Module: crate::utils
// Provides: {"create_symlink"}
// Dependencies: {}
pub fn create_symlink < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) -> Result < () , String > { # [cfg (windows)] let symlink = std :: os :: windows :: fs :: symlink_file ; # [cfg (not (windows))] let symlink = std :: os :: unix :: fs :: symlink ; symlink (& original , & link) . map_err (| err | { format ! ("failed to create a symlink `{}` to `{}`: {:?}" , original . as_ref () . display () , link . as_ref () . display () , err ,) }) }
};
}
