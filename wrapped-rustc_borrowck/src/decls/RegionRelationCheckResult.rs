macro_rules! RegionRelationCheckResult {
    () => {
        # [doc = " When we have an unmet lifetime constraint, we try to propagate it outward (e.g. to a closure"] # [doc = " environment). If we can't, it is an error."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum RegionRelationCheckResult { Ok , Propagated , Error , }
    };
}

RegionRelationCheckResult!()