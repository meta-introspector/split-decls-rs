macro_rules! deps {
    () => {
        RegionConstraintCollector!();
        ConstVidKey!();
        TyVidSubKey!();
        ProjectionCache!();
        RegionVidKey!();
        UndoLog!();
        TyVidEqKey!();
    };
}

macro_rules! macro_198 {
    () => {
        deps!();
        impl_from ! { RegionConstraintCollector (region_constraints :: UndoLog <'tcx >) , TypeVariables (sv :: UndoLog < ut :: Delegate < type_variable :: TyVidEqKey <'tcx >>>) , TypeVariables (sv :: UndoLog < ut :: Delegate < type_variable :: TyVidSubKey >>) , TypeVariables (type_variable :: UndoLog <'tcx >) , IntUnificationTable (sv :: UndoLog < ut :: Delegate < ty :: IntVid >>) , FloatUnificationTable (sv :: UndoLog < ut :: Delegate < ty :: FloatVid >>) , ConstUnificationTable (sv :: UndoLog < ut :: Delegate < ConstVidKey <'tcx >>>) , RegionUnificationTable (sv :: UndoLog < ut :: Delegate < RegionVidKey <'tcx >>>) , ProjectionCache (traits :: UndoLog <'tcx >) , }
    };
}

macro_198!();