macro_rules! LifetimeKind {
    () => {
        # [doc = " The kind of lifetime we are completing."] # [derive (Debug)] pub (crate) enum LifetimeKind { LifetimeParam , Lifetime { in_lifetime_param_bound : bool , def : Option < hir :: GenericDef > } , LabelRef , LabelDef , }
    };
}

LifetimeKind!();