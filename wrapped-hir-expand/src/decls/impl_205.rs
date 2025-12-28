macro_rules! deps {
    () => {
        SyntaxFixupUndoInfo!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl SyntaxFixupUndoInfo { pub (crate) const NONE : Self = SyntaxFixupUndoInfo { original : None } ; }
    };
}

impl_205!()