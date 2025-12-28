macro_rules! deps {
    () => {
        ClassSetBinaryOp!();
        ClassSetItem!();
    };
}

macro_rules! ClassSet {
    () => {
        deps!();
        # [doc = " A character class set."] # [doc = ""] # [doc = " This type corresponds to the internal structure of a bracketed character"] # [doc = " class. That is, every bracketed character is one of two types: a union of"] # [doc = " items (literals, ranges, other bracketed classes) or a tree of binary set"] # [doc = " operations."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassSet { # [doc = " An item, which can be a single literal, range, nested character class"] # [doc = " or a union of items."] Item (ClassSetItem) , # [doc = " A single binary operation (i.e., &&, -- or ~~)."] BinaryOp (ClassSetBinaryOp) , }
    };
}

ClassSet!();