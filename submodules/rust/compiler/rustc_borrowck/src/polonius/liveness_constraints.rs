mkuse!{use std :: collections :: BTreeMap ;}
mkuse!{use rustc_index :: bit_set :: SparseBitMatrix ;}
mkuse!{use rustc_middle :: mir :: { Body , Location } ;}
mkuse!{use rustc_middle :: ty :: relate :: { self , Relate , RelateResult , TypeRelation } ;}
mkuse!{use rustc_middle :: ty :: { self , RegionVid , Ty , TyCtxt , TypeVisitable } ;}
mkuse!{use rustc_mir_dataflow :: points :: PointIndex ;}
mkuse!{use super :: { ConstraintDirection , LocalizedOutlivesConstraint , LocalizedOutlivesConstraintSet , PoloniusLivenessContext , } ;}
mkuse!{use crate :: region_infer :: values :: LivenessValues ;}
mkuse!{use crate :: universal_regions :: UniversalRegions ;}
mkitem!{mkimpl!{impl PoloniusLivenessContext { # [doc = " Record the variance of each region contained within the given value."] pub (crate) fn record_live_region_variance < 'tcx > (& mut self , tcx : TyCtxt < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , value : impl TypeVisitable < TyCtxt < 'tcx > > + Relate < TyCtxt < 'tcx > > ,) { let mut extractor = VarianceExtractor { tcx , ambient_variance : ty :: Variance :: Covariant , directions : & mut self . live_region_variances , universal_regions , } ; extractor . relate (value , value) . expect ("Can't have a type error relating to itself") ; } }}}

macro_rules! create_liveness_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_liveness_constraints in module {}", module_path!());
    };
}

mkfn!{
    create_liveness_constraints_introspect!();
    # [doc = " Propagate loans throughout the CFG: for each statement in the MIR, create localized outlives"] # [doc = " constraints for loans that are propagated to the next statements."] pub (super) fn create_liveness_constraints < 'tcx > (body : & Body < 'tcx > , liveness : & LivenessValues , live_regions : & SparseBitMatrix < PointIndex , RegionVid > , live_region_variances : & BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & UniversalRegions < 'tcx > , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { for (block , bb) in body . basic_blocks . iter_enumerated () { let statement_count = bb . statements . len () ; for statement_index in 0 ..= statement_count { let current_location = Location { block , statement_index } ; let current_point = liveness . point_from_location (current_location) ; if statement_index < statement_count { let next_location = Location { block , statement_index : statement_index + 1 } ; let next_point = liveness . point_from_location (next_location) ; propagate_loans_between_points (current_point , next_point , live_regions , live_region_variances , universal_regions , localized_outlives_constraints ,) ; } else { for successor_block in bb . terminator () . successors () { let next_location = Location { block : successor_block , statement_index : 0 } ; let next_point = liveness . point_from_location (next_location) ; propagate_loans_between_points (current_point , next_point , live_regions , live_region_variances , universal_regions , localized_outlives_constraints ,) ; } } } } }
}

macro_rules! propagate_loans_between_points_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function propagate_loans_between_points in module {}", module_path!());
    };
}

mkfn!{
    propagate_loans_between_points_introspect!();
    # [doc = " Propagate loans within a region between two points in the CFG, if that region is live at both"] # [doc = " the source and target points."] fn propagate_loans_between_points (current_point : PointIndex , next_point : PointIndex , live_regions : & SparseBitMatrix < PointIndex , RegionVid > , live_region_variances : & BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & UniversalRegions < '_ > , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { for region in universal_regions . universal_regions_iter () { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; } let Some (next_live_regions) = live_regions . row (next_point) else { return ; } ; for region in next_live_regions . iter () { if let Some (& direction) = live_region_variances . get (& region) { add_liveness_constraint (region , current_point , next_point , direction , localized_outlives_constraints ,) ; } else { let fallback = ConstraintDirection :: Bidirectional ; add_liveness_constraint (region , current_point , next_point , fallback , localized_outlives_constraints ,) ; } } }
}

macro_rules! add_liveness_constraint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_liveness_constraint in module {}", module_path!());
    };
}

mkfn!{
    add_liveness_constraint_introspect!();
    # [doc = " Adds `LocalizedOutlivesConstraint`s between two connected points, according to the given edge"] # [doc = " direction."] fn add_liveness_constraint (region : RegionVid , current_point : PointIndex , next_point : PointIndex , direction : ConstraintDirection , localized_outlives_constraints : & mut LocalizedOutlivesConstraintSet ,) { match direction { ConstraintDirection :: Forward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; } ConstraintDirection :: Backward => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } ConstraintDirection :: Bidirectional => { localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : current_point , target : region , to : next_point , }) ; localized_outlives_constraints . push (LocalizedOutlivesConstraint { source : region , from : next_point , target : region , to : current_point , }) ; } } }
}
mkitem!{mkstruct!{# [doc = " Extracts variances for regions contained within types. Follows the same structure as"] # [doc = " `rustc_infer`'s `Generalizer`: we try to relate a type with itself to track and extract the"] # [doc = " variances of regions."] struct VarianceExtractor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , ambient_variance : ty :: Variance , directions : & 'a mut BTreeMap < RegionVid , ConstraintDirection > , universal_regions : & 'a UniversalRegions < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > VarianceExtractor < '_ , 'tcx > { fn record_variance (& mut self , region : ty :: Region < 'tcx > , variance : ty :: Variance) { if region . is_bound () { return ; } if region . is_erased () { return ; } let direction = match variance { ty :: Covariant => ConstraintDirection :: Forward , ty :: Contravariant => ConstraintDirection :: Backward , ty :: Invariant => ConstraintDirection :: Bidirectional , ty :: Bivariant => { return ; } } ; let region = self . universal_regions . to_region_vid (region) ; self . directions . entry (region) . and_modify (| entry | { if entry != & direction { * entry = ConstraintDirection :: Bidirectional ; } }) . or_insert (direction) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for VarianceExtractor < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_ambient_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; let r = self . relate (a , b) ? ; self . ambient_variance = old_ambient_variance ; Ok (r) } fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { assert_eq ! (a , b) ; relate :: structurally_relate_tys (self , a , b) } fn regions (& mut self , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { assert_eq ! (a , b) ; self . record_variance (a , self . ambient_variance) ; Ok (a) } fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { assert_eq ! (a , b) ; relate :: structurally_relate_consts (self , a , b) } fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , _ : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { self . relate (a . skip_binder () , a . skip_binder ()) ? ; Ok (a) } }}}