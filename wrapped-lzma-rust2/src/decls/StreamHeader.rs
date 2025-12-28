macro_rules! deps {
    () => {
        CheckType!();
    };
}

macro_rules! StreamHeader {
    () => {
        deps!();
        # [derive (Debug)] struct StreamHeader { pub check_type : CheckType , }
    };
}

StreamHeader!()