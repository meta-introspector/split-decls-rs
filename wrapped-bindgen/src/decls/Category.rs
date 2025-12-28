macro_rules! deps {
    () => {
        Interface!();
        Class!();
        Delegate!();
    };
}

macro_rules! Category {
    () => {
        deps!();
        # [derive (PartialEq)] enum Category { Interface , Class , Enum , Struct , Delegate , Attribute , }
    };
}

Category!();