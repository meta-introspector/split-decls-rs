macro_rules! AstOwner {
    () => {
        # [derive (Copy , Clone)] enum AstOwner < 'a > { NonOwner , Crate (& 'a ast :: Crate) , Item (& 'a ast :: Item) , AssocItem (& 'a ast :: AssocItem , visit :: AssocCtxt) , ForeignItem (& 'a ast :: ForeignItem) , }
    };
}

AstOwner!();