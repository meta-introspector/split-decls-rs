macro_rules! SlotSize {
    () => {
        enum SlotSize { Bytes8 = 8 , Bytes4 = 4 , }
    };
}

SlotSize!()