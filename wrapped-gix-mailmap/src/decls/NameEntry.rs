macro_rules! deps {
    () => {
        EncodedString!();
    };
}

macro_rules! NameEntry {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct NameEntry { pub (crate) new_name : Option < BString > , pub (crate) new_email : Option < BString > , pub (crate) old_name : EncodedString , }
    };
}

NameEntry!()