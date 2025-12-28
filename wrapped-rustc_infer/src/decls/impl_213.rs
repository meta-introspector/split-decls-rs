macro_rules! deps {
    () => {
        TyVidSubKey!();
        UndoLog!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) -> Self { UndoLog :: SubRelation (l) } }
    };
}

impl_213!()