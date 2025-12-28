macro_rules! deps {
    () => {
        FlattenOk!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        # [doc = " Only the iterator being flattened needs to implement [`FusedIterator`]."] impl < I , T , E > FusedIterator for FlattenOk < I , T , E > where I : FusedIterator < Item = Result < T , E > > , T : IntoIterator , { }
    };
}

impl_234!()