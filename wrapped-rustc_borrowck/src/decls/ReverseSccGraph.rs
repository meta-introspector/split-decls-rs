macro_rules! ReverseSccGraph {
    () => {
        pub (crate) struct ReverseSccGraph { graph : VecGraph < ConstraintSccIndex > , # [doc = " For each SCC, the range of `universal_regions` that use that SCC as"] # [doc = " their value."] scc_regions : FxIndexMap < ConstraintSccIndex , Range < usize > > , # [doc = " All of the universal regions, in grouped so that `scc_regions` can"] # [doc = " index into here."] universal_regions : Vec < RegionVid > , }
    };
}

ReverseSccGraph!();