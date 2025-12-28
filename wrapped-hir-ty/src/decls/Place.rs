macro_rules! deps {
    () => {
        ProjectionId!();
        LocalId!();
    };
}

macro_rules! Place {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Place < 'db > { pub local : LocalId < 'db > , pub projection : ProjectionId , }
    };
}

Place!()