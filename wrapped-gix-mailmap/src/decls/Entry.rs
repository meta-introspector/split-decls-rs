macro_rules! Entry {
    () => {
        # [doc = " An typical entry of a mailmap, which always contains an `old_email` by which"] # [doc = " the mapping is performed to replace the given `new_name` and `new_email`."] # [doc = ""] # [doc = " Optionally, `old_name` is also used for lookup."] # [doc = ""] # [doc = " Typically created by [parse()]."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy , Default)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Entry < 'a > { # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The name to map to."] pub (crate) new_name : Option < & 'a BStr > , # [doc = " The email map to."] pub (crate) new_email : Option < & 'a BStr > , # [doc = " The name to look for and replace."] pub (crate) old_name : Option < & 'a BStr > , # [doc = " The email to look for and replace."] pub (crate) old_email : & 'a BStr , }
    };
}

Entry!()