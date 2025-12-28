macro_rules! deps {
    () => {
        TupleId!();
    };
}

macro_rules! TupleFieldId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TupleFieldId { pub tuple : TupleId , pub index : u32 , }
    };
}

TupleFieldId!();