macro_rules! deps {
    () => {
        InvariantOpaque!();
    };
}

macro_rules! OperandBundle {
    () => {
        deps!();
        # [doc = " Opaque pointee of `LLVMOperandBundleRef`."] # [repr (C)] pub (crate) struct OperandBundle < 'a > (InvariantOpaque < 'a >) ;
    };
}

OperandBundle!();