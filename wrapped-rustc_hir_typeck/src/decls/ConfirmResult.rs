macro_rules! deps {
    () => {
        MethodCallee!();
    };
}

macro_rules! ConfirmResult {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ConfirmResult < 'tcx > { pub callee : MethodCallee < 'tcx > , pub illegal_sized_bound : Option < Span > , }
    };
}

ConfirmResult!()