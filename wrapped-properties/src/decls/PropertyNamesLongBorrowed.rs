macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
    };
}

macro_rules! PropertyNamesLongBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed wrapper around property value name-to-enum data, returned by"] # [doc = " [`PropertyNamesLong::as_borrowed()`]. More efficient to query."] # [derive (Debug)] pub struct PropertyNamesLongBorrowed < 'a , T : NamedEnumeratedProperty > { map : & 'a T :: DataStructLongBorrowed < 'a > , }
    };
}

PropertyNamesLongBorrowed!();