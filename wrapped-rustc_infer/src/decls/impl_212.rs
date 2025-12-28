macro_rules! deps {
    () => {
        UndoLog!();
        TyVidEqKey!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        # [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) -> Self { UndoLog :: EqRelation (l) } }
    };
}

impl_212!()