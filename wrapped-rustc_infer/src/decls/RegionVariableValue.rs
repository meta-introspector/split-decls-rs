macro_rules! RegionVariableValue {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) enum RegionVariableValue < 'tcx > { Known { value : ty :: Region < 'tcx > } , Unknown { universe : ty :: UniverseIndex } , }
    };
}

RegionVariableValue!()