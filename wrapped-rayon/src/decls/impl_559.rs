macro_rules! deps {
    () => {
        FoldWith!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl < I , U , F > FoldWith < I , U , F > { pub (super) fn new (base : I , item : U , fold_op : F) -> Self { FoldWith { base , item , fold_op , } } }
    };
}

impl_559!();