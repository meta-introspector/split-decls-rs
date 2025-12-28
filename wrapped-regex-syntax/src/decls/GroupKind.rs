macro_rules! deps {
    () => {
        Flags!();
        CaptureName!();
    };
}

macro_rules! GroupKind {
    () => {
        deps!();
        # [doc = " The kind of a group."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum GroupKind { # [doc = " `(a)`"] CaptureIndex (u32) , # [doc = " `(?<name>a)` or `(?P<name>a)`"] CaptureName { # [doc = " True if the `?P<` syntax is used and false if the `?<` syntax is used."] starts_with_p : bool , # [doc = " The capture name."] name : CaptureName , } , # [doc = " `(?:a)` and `(?i:a)`"] NonCapturing (Flags) , }
    };
}

GroupKind!()