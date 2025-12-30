// Generated macro for localize_statement_constraint (function)
macro_rules! Depcrate_polonius_typeck_constraintslocalize_statement_constraint {
() => {
// Module: crate::polonius::typeck_constraints
// Provides: {"localize_statement_constraint"}
// Dependencies: {}
# [doc = " For a given outlives constraint arising from a MIR statement, localize the constraint with the"] # [doc = " needed CFG `from`-`to` intra-block nodes."] fn localize_statement_constraint < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , stmt : & Statement < 'tcx > , outlives_constraint : & OutlivesConstraint < 'tcx > , current_point : PointIndex , universal_regions : & UniversalRegions < 'tcx > ,) -> LocalizedOutlivesConstraint { match & stmt . kind { StatementKind :: Assign (box (lhs , rhs)) => { debug_assert ! ({ let mut lhs_regions = FxHashSet :: default () ; tcx . for_each_free_region (lhs , | region | { let region = universal_regions . to_region_vid (region) ; lhs_regions . insert (region) ; }) ; let mut rhs_regions = FxHashSet :: default () ; tcx . for_each_free_region (rhs , | region | { let region = universal_regions . to_region_vid (region) ; rhs_regions . insert (region) ; }) ; lhs_regions . is_disjoint (& rhs_regions) } , "there should be no common regions between the LHS and RHS of an assignment") ; let lhs_ty = body . local_decls [lhs . local] . ty ; let successor_point = current_point ; compute_constraint_direction (tcx , outlives_constraint , & lhs_ty , current_point , successor_point , universal_regions ,) } _ => { LocalizedOutlivesConstraint { source : outlives_constraint . sup , from : current_point , target : outlives_constraint . sub , to : current_point , } } } }
};
}
