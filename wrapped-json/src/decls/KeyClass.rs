macro_rules! deps {
    () => {
        RawValue!();
        Number!();
        Map!();
    };
}

macro_rules! KeyClass {
    () => {
        deps!();
        enum KeyClass { Map (String) , # [cfg (feature = "arbitrary_precision")] Number , # [cfg (feature = "raw_value")] RawValue , }
    };
}

KeyClass!()