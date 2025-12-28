macro_rules! deps {
    () => {
        OperandBundle!();
    };
}

macro_rules! OperandBundleBox {
    () => {
        deps!();
        # [doc = " Owning pointer to an [`OperandBundle`] that will dispose of the bundle"] # [doc = " when dropped."] pub (crate) struct OperandBundleBox < 'a > { raw : ptr :: NonNull < OperandBundle < 'a > > , }
    };
}

OperandBundleBox!();