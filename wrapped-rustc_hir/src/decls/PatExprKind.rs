macro_rules! deps {
    () => {
        Lit!();
        Path!();
        QPath!();
        ConstBlock!();
    };
}

macro_rules! PatExprKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum PatExprKind < 'hir > { Lit { lit : Lit , negated : bool , } , ConstBlock (ConstBlock) , # [doc = " A path pattern for a unit struct/variant or a (maybe-associated) constant."] Path (QPath < 'hir >) , }
    };
}

PatExprKind!()