macro_rules! deps {
    () => {
        ByteSet!();
    };
}

macro_rules! BitSet {
    () => {
        deps!();
        # [doc = " The representation of a byte set. Split out so that we can define a"] # [doc = " convenient Debug impl for it while keeping \"ByteSet\" in the output."] # [derive (Clone , Copy , Default , Eq , PartialEq)] struct BitSet ([u128 ; 2]) ;
    };
}

BitSet!()