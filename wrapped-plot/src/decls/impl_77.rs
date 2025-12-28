macro_rules! deps {
    () => {
        Style!();
        ErrorBar!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < X , Y , L , H > ErrorBar < X , Y , L , H > { fn style (& self) -> Style { match * self { ErrorBar :: XErrorBars { .. } => Style :: XErrorBars , ErrorBar :: XErrorLines { .. } => Style :: XErrorLines , ErrorBar :: YErrorBars { .. } => Style :: YErrorBars , ErrorBar :: YErrorLines { .. } => Style :: YErrorLines , } } }
    };
}

impl_77!()