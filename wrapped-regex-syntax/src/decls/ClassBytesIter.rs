macro_rules! deps {
    () => {
        IntervalSetIter!();
        ClassBytesRange!();
    };
}

macro_rules! ClassBytesIter {
    () => {
        deps!();
        # [doc = " An iterator over all ranges in a byte character class."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the underlying class."] # [derive (Debug)] pub struct ClassBytesIter < 'a > (IntervalSetIter < 'a , ClassBytesRange >) ;
    };
}

ClassBytesIter!();