macro_rules! deps {
    () => {
        ClassUnicodeRange!();
        IntervalSet!();
    };
}

macro_rules! ClassUnicode {
    () => {
        deps!();
        # [doc = " A set of characters represented by Unicode scalar values."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ClassUnicode { set : IntervalSet < ClassUnicodeRange > , }
    };
}

ClassUnicode!()