macro_rules! AssocItemQSelf {
    () => {
        # [doc = " The \"qualified self\" of an associated item path."] # [doc = ""] # [doc = " For diagnostic purposes only."] enum AssocItemQSelf { Trait (DefId) , TyParam (LocalDefId , Span) , SelfTyAlias , }
    };
}

AssocItemQSelf!();