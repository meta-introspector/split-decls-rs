macro_rules! deps {
    () => {
        TypeVariableOrigin!();
    };
}

macro_rules! TypeVariableData {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct TypeVariableData { origin : TypeVariableOrigin , }
    };
}

TypeVariableData!();