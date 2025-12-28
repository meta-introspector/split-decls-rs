macro_rules! ReturnHint {
    () => {
        # [derive (Clone , Debug , PartialEq)] pub enum ReturnHint { None , Query (usize , usize) , QueryOptional (usize , usize) , ResultValue , ResultVoid , ReturnStruct , ReturnValue , }
    };
}

ReturnHint!();