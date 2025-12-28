macro_rules! deps {
    () => {
        ClassSetItem!();
        Span!();
    };
}

macro_rules! ClassSetUnion {
    () => {
        deps!();
        # [doc = " A union of items inside a character class set."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassSetUnion { # [doc = " The span of the items in this operation. e.g., the `a-z0-9` in"] # [doc = " `[^a-z0-9]`"] pub span : Span , # [doc = " The sequence of items that make up this union."] pub items : Vec < ClassSetItem > , }
    };
}

ClassSetUnion!();