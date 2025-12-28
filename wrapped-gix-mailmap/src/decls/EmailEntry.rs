macro_rules! deps {
    () => {
        NameEntry!();
        EncodedString!();
    };
}

macro_rules! EmailEntry {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct EmailEntry { pub (crate) new_name : Option < BString > , pub (crate) new_email : Option < BString > , pub (crate) old_email : EncodedString , pub (crate) entries_by_old_name : Vec < NameEntry > , }
    };
}

EmailEntry!()