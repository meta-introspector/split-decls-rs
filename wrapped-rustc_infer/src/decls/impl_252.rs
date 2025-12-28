macro_rules! deps {
    () => {
        TypeVariableTable!();
        InferCtxtInner!();
        TypeOutlivesConstraint!();
        TypeVariableValue!();
        ConstVidKey!();
        ProjectionCache!();
        OpaqueTypeTable!();
        RegionConstraintCollector!();
        UnificationTable!();
        InferCtxtUndoLogs!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < 'tcx > InferCtxtInner < 'tcx > { fn new () -> InferCtxtInner < 'tcx > { InferCtxtInner { undo_log : InferCtxtUndoLogs :: default () , projection_cache : Default :: default () , type_variable_storage : Default :: default () , const_unification_storage : Default :: default () , int_unification_storage : Default :: default () , float_unification_storage : Default :: default () , region_constraint_storage : Some (Default :: default ()) , region_obligations : Default :: default () , region_assumptions : Default :: default () , hir_typeck_potentially_region_dependent_goals : Default :: default () , opaque_type_storage : Default :: default () , } } # [inline] pub fn region_obligations (& self) -> & [TypeOutlivesConstraint < 'tcx >] { & self . region_obligations } # [inline] pub fn region_assumptions (& self) -> & [ty :: ArgOutlivesPredicate < 'tcx >] { & self . region_assumptions } # [inline] pub fn projection_cache (& mut self) -> traits :: ProjectionCache < '_ , 'tcx > { self . projection_cache . with_log (& mut self . undo_log) } # [inline] fn try_type_variables_probe_ref (& self , vid : ty :: TyVid ,) -> Option < & type_variable :: TypeVariableValue < 'tcx > > { self . type_variable_storage . eq_relations_ref () . try_probe_value (vid) } # [inline] fn type_variables (& mut self) -> type_variable :: TypeVariableTable < '_ , 'tcx > { self . type_variable_storage . with_log (& mut self . undo_log) } # [inline] pub fn opaque_types (& mut self) -> opaque_types :: OpaqueTypeTable < '_ , 'tcx > { self . opaque_type_storage . with_log (& mut self . undo_log) } # [inline] fn int_unification_table (& mut self) -> UnificationTable < '_ , 'tcx , ty :: IntVid > { self . int_unification_storage . with_log (& mut self . undo_log) } # [inline] fn float_unification_table (& mut self) -> UnificationTable < '_ , 'tcx , ty :: FloatVid > { self . float_unification_storage . with_log (& mut self . undo_log) } # [inline] fn const_unification_table (& mut self) -> UnificationTable < '_ , 'tcx , ConstVidKey < 'tcx > > { self . const_unification_storage . with_log (& mut self . undo_log) } # [inline] pub fn unwrap_region_constraints (& mut self) -> RegionConstraintCollector < '_ , 'tcx > { self . region_constraint_storage . as_mut () . expect ("region constraints already solved") . with_log (& mut self . undo_log) } }
    };
}

impl_252!();