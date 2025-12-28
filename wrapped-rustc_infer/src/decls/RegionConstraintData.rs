macro_rules! deps {
    () => {
        Verify!();
        Constraint!();
        SubregionOrigin!();
    };
}

macro_rules! RegionConstraintData {
    () => {
        deps!();
        # [doc = " The full set of region constraints gathered up by the collector."] # [doc = " Describes constraints between the region variables and other"] # [doc = " regions, as well as other conditions that must be verified, or"] # [doc = " assumptions that can be made."] # [derive (Debug , Default , Clone)] pub struct RegionConstraintData < 'tcx > { # [doc = " Constraints of the form `A <= B`, where either `A` or `B` can"] # [doc = " be a region variable (or neither, as it happens)."] pub constraints : Vec < (Constraint < 'tcx > , SubregionOrigin < 'tcx >) > , # [doc = " A \"verify\" is something that we need to verify after inference"] # [doc = " is done, but which does not directly affect inference in any"] # [doc = " way."] # [doc = ""] # [doc = " An example is a `A <= B` where neither `A` nor `B` are"] # [doc = " inference variables."] pub verifys : Vec < Verify < 'tcx > > , }
    };
}

RegionConstraintData!()