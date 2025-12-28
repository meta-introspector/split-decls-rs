macro_rules! deps {
    () => {
        TzDataIndex!();
    };
}

macro_rules! TzDataIndexes {
    () => {
        deps!();
        # [doc = " Indexes of the `tzdata` file."] struct TzDataIndexes { indexes : Vec < TzDataIndex > , }
    };
}

TzDataIndexes!();