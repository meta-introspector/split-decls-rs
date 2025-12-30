// Generated macro for BrokenLinkCallback (trait)
macro_rules! Depcrate_parseBrokenLinkCallback {
() => {
// Module: crate::parse
// Provides: {"BrokenLinkCallback"}
// Dependencies: {}
# [doc = " Trait for broken link callbacks."] # [doc = ""] # [doc = " See [Parser::new_with_broken_link_callback]."] # [doc = " Automatically implemented for closures with the appropriate signature."] pub trait BrokenLinkCallback < 'input > { fn handle_broken_link (& mut self , link : BrokenLink < 'input > ,) -> Option < (CowStr < 'input > , CowStr < 'input >) > ; }
};
}
