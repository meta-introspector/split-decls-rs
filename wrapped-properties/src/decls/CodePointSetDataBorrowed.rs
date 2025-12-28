macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! CodePointSetDataBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`CodePointSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct CodePointSetDataBorrowed < 'a > { set : & 'a PropertyCodePointSet < 'a > , }
    };
}

CodePointSetDataBorrowed!();