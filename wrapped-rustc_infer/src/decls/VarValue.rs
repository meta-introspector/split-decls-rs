macro_rules! VarValue {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) enum VarValue < 'tcx > { # [doc = " Empty lifetime is for data that is never accessed. We tag the"] # [doc = " empty lifetime with a universe -- the idea is that we don't"] # [doc = " want `exists<'a> { forall<'b> { 'b: 'a } }` to be satisfiable."] # [doc = " Therefore, the `'empty` in a universe `U` is less than all"] # [doc = " regions visible from `U`, but not less than regions not visible"] # [doc = " from `U`."] Empty (ty :: UniverseIndex) , Value (Region < 'tcx >) , ErrorValue , }
    };
}

VarValue!();