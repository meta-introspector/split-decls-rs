macro_rules! deps {
    () => {
        RegionElement!();
        TypeTest!();
    };
}

macro_rules! RegionErrorKind {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) enum RegionErrorKind < 'tcx > { # [doc = " A generic bound failure for a type test (`T: 'a`)."] TypeTestError { type_test : TypeTest < 'tcx > } , # [doc = " Higher-ranked subtyping error."] BoundUniversalRegionError { # [doc = " The placeholder free region."] longer_fr : RegionVid , # [doc = " The region element that erroneously must be outlived by `longer_fr`."] error_element : RegionElement , # [doc = " The placeholder region."] placeholder : ty :: PlaceholderRegion , } , # [doc = " Any other lifetime error."] RegionError { # [doc = " The origin of the region."] fr_origin : NllRegionVariableOrigin , # [doc = " The region that should outlive `shorter_fr`."] longer_fr : RegionVid , # [doc = " The region that should be shorter, but we can't prove it."] shorter_fr : RegionVid , # [doc = " Indicates whether this is a reported error. We currently only report the first error"] # [doc = " encountered and leave the rest unreported so as not to overwhelm the user."] is_reported : bool , } , }
    };
}

RegionErrorKind!()