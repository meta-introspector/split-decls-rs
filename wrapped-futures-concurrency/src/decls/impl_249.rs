macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < Fut > JoinTrait for Vec < Fut > where Fut : IntoFuture , { type Output = Vec < Fut :: Output > ; type Future = Join < Fut :: IntoFuture > ; fn join (self) -> Self :: Future { Join :: new (self . into_iter () . map (IntoFuture :: into_future) . collect ()) } }
    };
}

impl_249!();