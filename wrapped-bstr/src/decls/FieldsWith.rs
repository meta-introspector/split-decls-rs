macro_rules! deps {
    () => {
        CharIndices!();
    };
}

macro_rules! FieldsWith {
    () => {
        deps!();
        # [doc = " An iterator over fields in the byte string, separated by a predicate over"] # [doc = " codepoints."] # [doc = ""] # [doc = " This iterator splits a byte string based on its predicate function such"] # [doc = " that the elements returned are separated by contiguous runs of codepoints"] # [doc = " for which the predicate returns true."] # [doc = ""] # [doc = " `'a` is the lifetime of the byte string being split, while `F` is the type"] # [doc = " of the predicate, i.e., `FnMut(char) -> bool`."] # [derive (Clone , Debug)] pub struct FieldsWith < 'a , F > { f : F , bytes : & 'a [u8] , chars : CharIndices < 'a > , }
    };
}

FieldsWith!();