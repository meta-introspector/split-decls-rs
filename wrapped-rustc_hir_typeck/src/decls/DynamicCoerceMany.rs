macro_rules! deps {
    () => {
        CoerceMany!();
    };
}

macro_rules! DynamicCoerceMany {
    () => {
        deps!();
        # [doc = " The type of a `CoerceMany` that is storing up the expressions into"] # [doc = " a buffer. We use this in `check/mod.rs` for things like `break`."] pub (crate) type DynamicCoerceMany < 'tcx > = CoerceMany < 'tcx , 'tcx , & 'tcx hir :: Expr < 'tcx > > ;
    };
}

DynamicCoerceMany!()