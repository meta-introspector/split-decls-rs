macro_rules! deps {
    () => {
        PropertyCodePointMap!();
    };
}

macro_rules! CodePointMapDataBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`CodePointSetData::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct CodePointMapDataBorrowed < 'a , T : TrieValue > { map : & 'a PropertyCodePointMap < 'a , T > , }
    };
}

CodePointMapDataBorrowed!()