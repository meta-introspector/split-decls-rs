macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! MovedOutOfRef {
    () => {
        deps!();
        # [derive (Debug)] pub struct MovedOutOfRef < 'db > { pub ty : Type < 'db > , pub span : InFile < SyntaxNodePtr > , }
    };
}

MovedOutOfRef!();