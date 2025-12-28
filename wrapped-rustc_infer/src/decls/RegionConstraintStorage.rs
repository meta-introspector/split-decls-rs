macro_rules! deps {
    () => {
        CombineMap!();
        RegionVariableOrigin!();
        RegionVidKey!();
        RegionConstraintData!();
        RegionVariableInfo!();
    };
}

macro_rules! RegionConstraintStorage {
    () => {
        deps!();
        # [derive (Clone , Default)] pub struct RegionConstraintStorage < 'tcx > { # [doc = " For each `RegionVid`, the corresponding `RegionVariableOrigin`."] pub (super) var_infos : IndexVec < RegionVid , RegionVariableInfo > , pub (super) data : RegionConstraintData < 'tcx > , # [doc = " For a given pair of regions (R1, R2), maps to a region R3 that"] # [doc = " is designated as their LUB (edges R1 <= R3 and R2 <= R3"] # [doc = " exist). This prevents us from making many such regions."] lubs : CombineMap < 'tcx > , # [doc = " For a given pair of regions (R1, R2), maps to a region R3 that"] # [doc = " is designated as their GLB (edges R3 <= R1 and R3 <= R2"] # [doc = " exist). This prevents us from making many such regions."] glbs : CombineMap < 'tcx > , # [doc = " When we add a R1 == R2 constraint, we currently add (a) edges"] # [doc = " R1 <= R2 and R2 <= R1 and (b) we unify the two regions in this"] # [doc = " table. You can then call `opportunistic_resolve_var` early"] # [doc = " which will map R1 and R2 to some common region (i.e., either"] # [doc = " R1 or R2). This is important when fulfillment, dropck and other such"] # [doc = " code is iterating to a fixed point, because otherwise we sometimes"] # [doc = " would wind up with a fresh stream of region variables that have been"] # [doc = " equated but appear distinct."] pub (super) unification_table : ut :: UnificationTableStorage < RegionVidKey < 'tcx > > , # [doc = " a flag set to true when we perform any unifications; this is used"] # [doc = " to micro-optimize `take_and_reset_data`"] any_unifications : bool , }
    };
}

RegionConstraintStorage!();