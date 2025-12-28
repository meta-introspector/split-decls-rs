macro_rules! AnnNode {
    () => {
        pub enum AnnNode < 'a > { Ident (& 'a Ident) , Name (& 'a Symbol) , Block (& 'a ast :: Block) , Item (& 'a ast :: Item) , SubItem (ast :: NodeId) , Expr (& 'a ast :: Expr) , Pat (& 'a ast :: Pat) , Crate (& 'a ast :: Crate) , }
    };
}

AnnNode!()