macro_rules! deps {
    () => {
        StatementKind!();
        MirSpan!();
    };
}

macro_rules! Statement {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub struct Statement < 'db > { pub kind : StatementKind < 'db > , pub span : MirSpan , }
    };
}

Statement!();