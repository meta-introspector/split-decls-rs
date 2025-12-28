macro_rules! deps {
    () => {
        AstIdentifiedAnn!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl pprust_ast :: PpAnn for AstIdentifiedAnn { fn pre (& self , s : & mut pprust_ast :: State < '_ > , node : pprust_ast :: AnnNode < '_ >) { if let pprust_ast :: AnnNode :: Expr (_) = node { s . popen () ; } } fn post (& self , s : & mut pprust_ast :: State < '_ > , node : pprust_ast :: AnnNode < '_ >) { match node { pprust_ast :: AnnNode :: Crate (_) | pprust_ast :: AnnNode :: Ident (_) | pprust_ast :: AnnNode :: Name (_) => { } pprust_ast :: AnnNode :: Item (item) => { s . s . space () ; s . synth_comment (item . id . to_string ()) } pprust_ast :: AnnNode :: SubItem (id) => { s . s . space () ; s . synth_comment (id . to_string ()) } pprust_ast :: AnnNode :: Block (blk) => { s . s . space () ; s . synth_comment (format ! ("block {}" , blk . id)) } pprust_ast :: AnnNode :: Expr (expr) => { s . s . space () ; s . synth_comment (expr . id . to_string ()) ; s . pclose () } pprust_ast :: AnnNode :: Pat (pat) => { s . s . space () ; s . synth_comment (format ! ("pat {}" , pat . id)) ; } } } }
    };
}

impl_13!();