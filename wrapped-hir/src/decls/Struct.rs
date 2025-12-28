macro_rules! Struct {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Struct { pub (crate) id : StructId , }
    };
}

Struct!();