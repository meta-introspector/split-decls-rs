macro_rules! TypeVariableValue {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) enum TypeVariableValue < 'tcx > { Known { value : Ty < 'tcx > } , Unknown { universe : ty :: UniverseIndex } , }
    };
}

TypeVariableValue!()