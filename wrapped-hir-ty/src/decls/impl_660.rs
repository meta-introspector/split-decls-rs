macro_rules! deps {
    () => {
        LayoutError!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        impl < F > From < LayoutCalculatorError < F > > for LayoutError { fn from (err : LayoutCalculatorError < F >) -> Self { LayoutError :: BadCalc (err . without_payload ()) } }
    };
}

impl_660!()