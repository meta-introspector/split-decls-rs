macro_rules! deps {
    () => {
        Attributes!();
        Source!();
        AttributeMatchGroup!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [doc = " Initialization"] impl Attributes { # [doc = " Create a new instance from an attribute match group that represents `globals`. It can more easily be created with"] # [doc = " [`AttributeMatchGroup::new_globals()`]."] # [doc = ""] # [doc = " * `globals` contribute first and consist of all globally available, static files."] # [doc = " * `info_attributes` is a path that should refer to `.git/info/attributes`, and it's not an error if the file doesn't exist."] # [doc = " * `case` is used to control case-sensitivity during matching."] # [doc = " * `source` specifies from where the directory-based attribute files should be loaded from."] pub fn new (globals : AttributeMatchGroup , info_attributes : Option < PathBuf > , source : Source , collection : gix_attributes :: search :: MetadataCollection ,) -> Self { Attributes { globals , stack : Default :: default () , info_attributes , source , collection , } } }
    };
}

impl_26!()