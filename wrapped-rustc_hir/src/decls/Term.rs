macro_rules! deps {
    () => {
        ConstArg!();
        Ty!();
    };
}

macro_rules! Term {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum Term < 'hir > { Ty (& 'hir Ty < 'hir >) , Const (& 'hir ConstArg < 'hir >) , }
    };
}

Term!();