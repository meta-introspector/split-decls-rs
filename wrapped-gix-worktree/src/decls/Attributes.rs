macro_rules! deps {
    () => {
        Source!();
        State!();
        AttributeMatchGroup!();
    };
}

macro_rules! Attributes {
    () => {
        deps!();
        # [doc = " State related to attributes associated with files in the repository."] # [derive (Default , Clone)] # [cfg (feature = "attributes")] pub struct Attributes { # [doc = " Attribute patterns which aren't tied to the repository root, hence are global, they contribute first."] globals : AttributeMatchGroup , # [doc = " Attribute patterns that match the currently set directory (in the stack)."] # [doc = ""] # [doc = " Note that the root-level file is always loaded, if present, followed by, the `$GIT_DIR/info/attributes`, if present, based"] # [doc = " on the location of the `info_attributes` file."] stack : AttributeMatchGroup , # [doc = " The first time we push the root, we have to load additional information from this file if it exists along with the root attributes"] # [doc = " file if possible, and keep them there throughout."] info_attributes : Option < std :: path :: PathBuf > , # [doc = " A lookup table to accelerate searches."] collection : gix_attributes :: search :: MetadataCollection , # [doc = " Where to read `.gitattributes` data from."] source : attributes :: Source , }
    };
}

Attributes!();