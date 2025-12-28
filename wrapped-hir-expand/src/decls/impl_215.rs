macro_rules! deps {
    () => {
        TransformTtAction!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl TransformTtAction < '_ > { fn remove () -> Self { Self :: ReplaceWith (tt :: TokenTreesView :: new (& [])) } }
    };
}

impl_215!()