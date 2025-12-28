macro_rules! TypeAscriptionTarget {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum TypeAscriptionTarget { Let (Option < ast :: Pat >) , FnParam (Option < ast :: Pat >) , RetType (Option < ast :: Expr >) , Const (Option < ast :: Expr >) , }
    };
}

TypeAscriptionTarget!();