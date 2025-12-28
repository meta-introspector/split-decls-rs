macro_rules! PathKind {
    () => {
        # [derive (Copy , Clone , PartialEq)] pub enum PathKind { Simple , Type , Expr , }
    };
}

PathKind!()