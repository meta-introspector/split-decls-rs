macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! CaptureName {
    () => {
        deps!();
        # [doc = " A capture name."] # [doc = ""] # [doc = " This corresponds to the name itself between the angle brackets in, e.g.,"] # [doc = " `(?P<foo>expr)`."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct CaptureName { # [doc = " The span of this capture name."] pub span : Span , # [doc = " The capture name."] pub name : String , # [doc = " The capture index."] pub index : u32 , }
    };
}

CaptureName!();