macro_rules! AttrId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct AttrId { id : u32 , }
    };
}

AttrId!()