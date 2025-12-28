macro_rules! deps {
    () => {
        FoldChunksWith!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        impl < I , U , F > FoldChunksWith < I , U , F > { # [doc = " Creates a new `FoldChunksWith` iterator"] pub (super) fn new (base : I , chunk_size : usize , item : U , fold_op : F) -> Self { FoldChunksWith { base , chunk_size , item , fold_op , } } }
    };
}

impl_576!()