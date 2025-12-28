macro_rules! deps {
    () => {
        UndoLog!();
        RegionConstraintCollector!();
        InferCtxtInner!();
        ProjectionCache!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        # [doc = " The Rollback trait defines how to rollback a particular action."] impl < 'tcx > Rollback < UndoLog < 'tcx > > for InferCtxtInner < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { UndoLog :: DuplicateOpaqueType => self . opaque_type_storage . pop_duplicate_entry () , UndoLog :: OpaqueTypes (key , idx) => self . opaque_type_storage . remove (key , idx) , UndoLog :: TypeVariables (undo) => self . type_variable_storage . reverse (undo) , UndoLog :: ConstUnificationTable (undo) => self . const_unification_storage . reverse (undo) , UndoLog :: IntUnificationTable (undo) => self . int_unification_storage . reverse (undo) , UndoLog :: FloatUnificationTable (undo) => self . float_unification_storage . reverse (undo) , UndoLog :: RegionConstraintCollector (undo) => { self . region_constraint_storage . as_mut () . unwrap () . reverse (undo) } UndoLog :: RegionUnificationTable (undo) => { self . region_constraint_storage . as_mut () . unwrap () . unification_table . reverse (undo) } UndoLog :: ProjectionCache (undo) => self . projection_cache . reverse (undo) , UndoLog :: PushTypeOutlivesConstraint => { let popped = self . region_obligations . pop () ; assert_matches ! (popped , Some (_) , "pushed region constraint but could not pop it") ; } UndoLog :: PushRegionAssumption => { let popped = self . region_assumptions . pop () ; assert_matches ! (popped , Some (_) , "pushed region assumption but could not pop it") ; } UndoLog :: PushHirTypeckPotentiallyRegionDependentGoal => { let popped = self . hir_typeck_potentially_region_dependent_goals . pop () ; assert_matches ! (popped , Some (_) , "pushed goal but could not pop it") ; } } } }
    };
}

impl_199!();