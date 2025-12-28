macro_rules! deps {
    () => {
        Curve!();
        Style!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < X , Y > Curve < X , Y > { fn style (& self) -> Style { match * self { Curve :: Dots { .. } => Style :: Dots , Curve :: Impulses { .. } => Style :: Impulses , Curve :: Lines { .. } => Style :: Lines , Curve :: LinesPoints { .. } => Style :: LinesPoints , Curve :: Points { .. } => Style :: Points , Curve :: Steps { .. } => Style :: Steps , } } }
    };
}

impl_60!();