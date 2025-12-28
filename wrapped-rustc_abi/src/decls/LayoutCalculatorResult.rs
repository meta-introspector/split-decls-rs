macro_rules! deps {
    () => {
        LayoutCalculatorError!();
        LayoutData!();
    };
}

macro_rules! LayoutCalculatorResult {
    () => {
        deps!();
        type LayoutCalculatorResult < FieldIdx , VariantIdx , F > = Result < LayoutData < FieldIdx , VariantIdx > , LayoutCalculatorError < F > > ;
    };
}

LayoutCalculatorResult!();