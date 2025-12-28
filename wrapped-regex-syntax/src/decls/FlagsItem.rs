macro_rules! deps {
    () => {
        FlagsItemKind!();
        Span!();
    };
}

macro_rules! FlagsItem {
    () => {
        deps!();
        # [doc = " A single item in a group of flags."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct FlagsItem { # [doc = " The span of this item."] pub span : Span , # [doc = " The kind of this item."] pub kind : FlagsItemKind , }
    };
}

FlagsItem!();