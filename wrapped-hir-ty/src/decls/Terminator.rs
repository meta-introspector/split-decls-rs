macro_rules! deps {
    () => {
        MirSpan!();
        TerminatorKind!();
    };
}

macro_rules! Terminator {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub struct Terminator < 'db > { pub span : MirSpan , pub kind : TerminatorKind < 'db > , }
    };
}

Terminator!();