macro_rules! SelfSource {
    () => {
        # [derive (Copy , Clone , Debug)] enum SelfSource < 'a > { QPath (& 'a hir :: Ty < 'a >) , MethodCall (& 'a hir :: Expr < 'a >) , }
    };
}

SelfSource!()