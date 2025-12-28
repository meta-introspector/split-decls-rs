macro_rules! deps {
    () => {
        TryFoldWith!();
    };
}

macro_rules! impl_906 {
    () => {
        deps!();
        impl < I , U : Try , F > TryFoldWith < I , U , F > { pub (super) fn new (base : I , item : U :: Output , fold_op : F) -> Self { TryFoldWith { base , item , fold_op , } } }
    };
}

impl_906!()