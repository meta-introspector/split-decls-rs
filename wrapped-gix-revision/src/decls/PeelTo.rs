macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! PeelTo {
    () => {
        deps!();
        # [doc = " Define where a tag object should be peeled to."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum PeelTo < 'a > { # [doc = " An object of the given kind."] ObjectKind (gix_object :: Kind) , # [doc = " Ensure the object at hand exists and is valid (actually without peeling it),"] # [doc = " without imposing any restrictions to its type."] # [doc = " The object needs to be looked up to assure that it is valid, but it doesn't need to be decoded."] ValidObject , # [doc = " Follow an annotated tag object recursively until an object is found."] RecursiveTagObject , # [doc = " The path to drill into as seen relative to the current tree-ish."] # [doc = ""] # [doc = " Note that the path can be relative, and `./` and `../` prefixes are seen as relative to the current"] # [doc = " working directory."] # [doc = ""] # [doc = " The path may be empty, which makes it refer to the tree at the current revision, similar to `^{tree}`."] # [doc = " Note that paths like `../` are valid and refer to a tree as seen relative to the current working directory."] Path (& 'a BStr) , }
    };
}

PeelTo!();