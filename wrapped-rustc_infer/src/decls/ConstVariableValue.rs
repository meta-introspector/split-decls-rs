macro_rules! deps {
    () => {
        ConstVariableOrigin!();
    };
}

macro_rules! ConstVariableValue {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub (crate) enum ConstVariableValue < 'tcx > { Known { value : ty :: Const < 'tcx > } , Unknown { origin : ConstVariableOrigin , universe : ty :: UniverseIndex } , }
    };
}

ConstVariableValue!()