macro_rules! deps {
    () => {
        FoldKind!();
    };
}

macro_rules! Fold {
    () => {
        deps!();
        # [derive (Debug)] pub struct Fold { pub range : TextRange , pub kind : FoldKind , }
    };
}

Fold!()