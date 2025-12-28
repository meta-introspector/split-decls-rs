macro_rules! deps {
    () => {
        ShorthandAssocTyCollector!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl hir :: intravisit :: Visitor < '_ > for ShorthandAssocTyCollector { fn visit_qpath (& mut self , qpath : & hir :: QPath < '_ > , id : hir :: HirId , _ : Span) { if let hir :: QPath :: TypeRelative (qself , _) = qpath && qself . as_generic_param () . is_some () { self . qselves . push (qself . span) ; } hir :: intravisit :: walk_qpath (self , qpath , id) } }
    };
}

impl_69!();