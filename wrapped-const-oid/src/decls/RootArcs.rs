macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! RootArcs {
    () => {
        deps!();
        # [doc = " Byte containing the first and second arcs of an OID."] # [doc = ""] # [doc = " This is represented this way in order to reduce the overall size of the"] # [doc = " [`ObjectIdentifier`] struct."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] struct RootArcs (u8) ;
    };
}

RootArcs!()