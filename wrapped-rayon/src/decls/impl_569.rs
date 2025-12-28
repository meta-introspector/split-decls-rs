macro_rules! deps {
    () => {
        FoldChunks!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        impl < I , ID , F > FoldChunks < I , ID , F > { # [doc = " Creates a new `FoldChunks` iterator"] pub (super) fn new (base : I , chunk_size : usize , identity : ID , fold_op : F) -> Self { FoldChunks { base , chunk_size , identity , fold_op , } } }
    };
}

impl_569!();