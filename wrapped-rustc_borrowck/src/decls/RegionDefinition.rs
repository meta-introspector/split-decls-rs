macro_rules! RegionDefinition {
    () => {
        # [derive (Debug)] pub (crate) struct RegionDefinition < 'tcx > { # [doc = " What kind of variable is this -- a free region? existential"] # [doc = " variable? etc. (See the `NllRegionVariableOrigin` for more"] # [doc = " info.)"] pub (crate) origin : NllRegionVariableOrigin , # [doc = " Which universe is this region variable defined in? This is"] # [doc = " most often `ty::UniverseIndex::ROOT`, but when we encounter"] # [doc = " forall-quantifiers like `for<'a> { 'a = 'b }`, we would create"] # [doc = " the variable for `'a` in a fresh universe that extends ROOT."] pub (crate) universe : ty :: UniverseIndex , # [doc = " If this is 'static or an early-bound region, then this is"] # [doc = " `Some(X)` where `X` is the name of the region."] pub (crate) external_name : Option < ty :: Region < 'tcx > > , }
    };
}

RegionDefinition!();