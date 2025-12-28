macro_rules! AttributeId {
    () => {
        # [doc = " A type to denote an id of an attribute assignment for uniquely identifying each attribute or assignment."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct AttributeId (pub usize) ;
    };
}

AttributeId!()