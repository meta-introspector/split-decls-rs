macro_rules! deps {
    () => {
        PreciseCapturingArgKind!();
        PreciseCapturingNonLifetimeArg!();
        Lifetime!();
    };
}

macro_rules! PreciseCapturingArg {
    () => {
        deps!();
        pub type PreciseCapturingArg < 'hir > = PreciseCapturingArgKind < & 'hir Lifetime , PreciseCapturingNonLifetimeArg > ;
    };
}

PreciseCapturingArg!();