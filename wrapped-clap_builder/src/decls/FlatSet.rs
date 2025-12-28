macro_rules! FlatSet {
    () => {
        # [doc = " Flat (Vec) backed set"] # [doc = ""] # [doc = " This preserves insertion order"] # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct FlatSet < T > { inner : Vec < T > , }
    };
}

FlatSet!()