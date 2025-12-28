macro_rules! deps {
    () => {
        OperandValueBuilder!();
        OperandRef!();
    };
}

macro_rules! OperandRefBuilder {
    () => {
        deps!();
        # [doc = " Allows building up an `OperandRef` by setting fields one at a time."] # [derive (Debug , Copy , Clone)] pub (super) struct OperandRefBuilder < 'tcx , V > { val : OperandValueBuilder < V > , layout : TyAndLayout < 'tcx > , }
    };
}

OperandRefBuilder!()