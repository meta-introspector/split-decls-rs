macro_rules! deps {
    () => {
        Target!();
        FullName!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " A fully owned backend agnostic reference"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Reference { # [doc = " The path to uniquely identify this ref within its store."] pub name : FullName , # [doc = " The target of the reference, either a symbolic reference by full name or a possibly intermediate object by its id."] pub target : Target , # [doc = " The fully peeled object to which this reference ultimately points to after following all symbolic refs and all annotated"] # [doc = " tags. Only guaranteed to be set after"] # [doc = " [`Reference::peel_to_id()`](crate::file::ReferenceExt::peel_to_id) was called or if this reference originated"] # [doc = " from a packed ref."] pub peeled : Option < ObjectId > , }
    };
}

Reference!();