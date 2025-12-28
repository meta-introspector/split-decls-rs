macro_rules! LayoutError {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum LayoutError { BadCalc (LayoutCalculatorError < () >) , HasErrorConst , HasErrorType , HasPlaceholder , InvalidSimdType , NotImplemented , RecursiveTypeWithoutIndirection , TargetLayoutNotAvailable , Unknown , UserReprTooSmall , }
    };
}

LayoutError!()