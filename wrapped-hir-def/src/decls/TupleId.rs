macro_rules! TupleId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TupleId (pub u32) ;
    };
}

TupleId!()