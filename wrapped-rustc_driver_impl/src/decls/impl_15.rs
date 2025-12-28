macro_rules! deps {
    () => {
        HirIdentifiedAnn!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'tcx > pprust_hir :: PpAnn for HirIdentifiedAnn < 'tcx > { fn nested (& self , state : & mut pprust_hir :: State < '_ > , nested : pprust_hir :: Nested) { self . tcx . nested (state , nested) } fn pre (& self , s : & mut pprust_hir :: State < '_ > , node : pprust_hir :: AnnNode < '_ >) { if let pprust_hir :: AnnNode :: Expr (_) = node { s . popen () ; } } fn post (& self , s : & mut pprust_hir :: State < '_ > , node : pprust_hir :: AnnNode < '_ >) { match node { pprust_hir :: AnnNode :: Name (_) => { } pprust_hir :: AnnNode :: Item (item) => { s . s . space () ; s . synth_comment (format ! ("hir_id: {}" , item . hir_id ())) ; } pprust_hir :: AnnNode :: SubItem (id) => { s . s . space () ; s . synth_comment (id . to_string ()) ; } pprust_hir :: AnnNode :: Block (blk) => { s . s . space () ; s . synth_comment (format ! ("block hir_id: {}" , blk . hir_id)) ; } pprust_hir :: AnnNode :: Expr (expr) => { s . s . space () ; s . synth_comment (format ! ("expr hir_id: {}" , expr . hir_id)) ; s . pclose () ; } pprust_hir :: AnnNode :: Pat (pat) => { s . s . space () ; s . synth_comment (format ! ("pat hir_id: {}" , pat . hir_id)) ; } pprust_hir :: AnnNode :: TyPat (pat) => { s . s . space () ; s . synth_comment (format ! ("ty pat hir_id: {}" , pat . hir_id)) ; } pprust_hir :: AnnNode :: Arm (arm) => { s . s . space () ; s . synth_comment (format ! ("arm hir_id: {}" , arm . hir_id)) ; } } } }
    };
}

impl_15!()