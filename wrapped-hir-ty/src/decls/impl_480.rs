macro_rules! deps {
    () => {
        BoundsFormattingCtx!();
        AliasTy!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl < 'db > BoundsFormattingCtx < 'db > { fn contains (& self , proj : & AliasTy < 'db >) -> bool { match self { BoundsFormattingCtx :: Entered { projection_tys_met } => { projection_tys_met . contains (proj) } BoundsFormattingCtx :: Exited => false , } } }
    };
}

impl_480!();