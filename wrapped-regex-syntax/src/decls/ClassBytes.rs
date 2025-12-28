macro_rules! deps {
    () => {
        ClassBytesRange!();
        IntervalSet!();
    };
}

macro_rules! ClassBytes {
    () => {
        deps!();
        # [doc = " A set of characters represented by arbitrary bytes."] # [doc = ""] # [doc = " Each byte corresponds to one character."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ClassBytes { set : IntervalSet < ClassBytesRange > , }
    };
}

ClassBytes!();