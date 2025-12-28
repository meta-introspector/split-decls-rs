macro_rules! Enum {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Enum { pub (crate) id : EnumId , }
    };
}

Enum!()