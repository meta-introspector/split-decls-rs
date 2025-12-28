macro_rules! Parameter {
    () => {
        # [derive (Clone , PartialEq , Eq , Hash , Debug)] pub (crate) struct Parameter (pub u32) ;
    };
}

Parameter!();