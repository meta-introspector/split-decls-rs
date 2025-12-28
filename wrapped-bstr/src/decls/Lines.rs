macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! Lines {
    () => {
        deps!();
        # [doc = " An iterator over all lines in a byte string, without their terminators."] # [doc = ""] # [doc = " For this iterator, the only line terminators recognized are `\\r\\n` and"] # [doc = " `\\n`."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being iterated over."] # [derive (Clone , Debug)] pub struct Lines < 'a > { it : LinesWithTerminator < 'a > , }
    };
}

Lines!();