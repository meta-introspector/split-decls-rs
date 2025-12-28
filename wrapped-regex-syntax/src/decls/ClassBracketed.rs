macro_rules! deps {
    () => {
        Span!();
        ClassSet!();
    };
}

macro_rules! ClassBracketed {
    () => {
        deps!();
        # [doc = " A bracketed character class, e.g., `[a-z0-9]`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct ClassBracketed { # [doc = " The span of this class."] pub span : Span , # [doc = " Whether this class is negated or not. e.g., `[a]` is not negated but"] # [doc = " `[^a]` is."] pub negated : bool , # [doc = " The type of this set. A set is either a normal union of things, e.g.,"] # [doc = " `[abc]` or a result of applying set operations, e.g., `[\\pL--c]`."] pub kind : ClassSet , }
    };
}

ClassBracketed!();