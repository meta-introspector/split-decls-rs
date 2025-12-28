macro_rules! deps {
    () => {
        IntervalSetIter!();
        ClassUnicodeRange!();
    };
}

macro_rules! ClassUnicodeIter {
    () => {
        deps!();
        # [doc = " An iterator over all ranges in a Unicode character class."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the underlying class."] # [derive (Debug)] pub struct ClassUnicodeIter < 'a > (IntervalSetIter < 'a , ClassUnicodeRange >) ;
    };
}

ClassUnicodeIter!();