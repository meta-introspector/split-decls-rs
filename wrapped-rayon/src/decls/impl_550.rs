macro_rules! deps {
    () => {
        Fold!();
    };
}

macro_rules! impl_550 {
    () => {
        deps!();
        impl < I , ID , F > Fold < I , ID , F > { pub (super) fn new (base : I , identity : ID , fold_op : F) -> Self { Fold { base , identity , fold_op , } } }
    };
}

impl_550!();