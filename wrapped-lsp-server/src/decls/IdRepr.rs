macro_rules! IdRepr {
    () => {
        # [derive (Debug , Serialize , Deserialize , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [serde (untagged)] enum IdRepr { I32 (i32) , String (String) , }
    };
}

IdRepr!();