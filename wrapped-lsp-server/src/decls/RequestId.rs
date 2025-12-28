macro_rules! deps {
    () => {
        IdRepr!();
    };
}

macro_rules! RequestId {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [serde (transparent)] pub struct RequestId (IdRepr) ;
    };
}

RequestId!();