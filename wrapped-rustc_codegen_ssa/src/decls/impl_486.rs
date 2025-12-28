macro_rules! deps {
    () => {
        Locals!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        # [doc = " To mutate locals, use `FunctionCx::overwrite_local` instead."] impl < 'tcx , V , Idx : ? Sized > ! IndexMut < Idx > for Locals < 'tcx , V > { }
    };
}

impl_486!();