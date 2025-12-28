macro_rules! deps {
    () => {
        PreciseCapturingArgKind!();
        Lifetime!();
        PreciseCapturingNonLifetimeArg!();
    };
}

macro_rules! PreciseCapturingArg {
    () => {
        deps!();
        pub type PreciseCapturingArg < 'hir > = PreciseCapturingArgKind < & 'hir Lifetime , PreciseCapturingNonLifetimeArg > ;
    };
}

PreciseCapturingArg!()