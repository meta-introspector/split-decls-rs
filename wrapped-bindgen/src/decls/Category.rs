macro_rules! deps {
    () => {
        Delegate!();
        Interface!();
        Class!();
    };
}

macro_rules! Category {
    () => {
        deps!();
        # [derive (PartialEq)] enum Category { Interface , Class , Enum , Struct , Delegate , Attribute , }
    };
}

Category!()