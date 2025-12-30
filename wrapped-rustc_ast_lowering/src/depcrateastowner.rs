// Generated macro for AstOwner (enum)
macro_rules! DepcrateAstOwner {
() => {
// Module: crate
// Provides: {"AstOwner"}
// Dependencies: {}
# [derive (Copy , Clone)] enum AstOwner < 'a > { NonOwner , Crate (& 'a ast :: Crate) , Item (& 'a ast :: Item) , AssocItem (& 'a ast :: AssocItem , visit :: AssocCtxt) , ForeignItem (& 'a ast :: ForeignItem) , }
};
}
