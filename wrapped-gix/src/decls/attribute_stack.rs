macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! attribute_stack {
    () => {
        deps!();
        # [doc = ""] pub mod attribute_stack { # [doc = " The error produced when setting up the attribute stack to query `gitattributes`."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("An attribute file could not be read")] Io (# [from] std :: io :: Error) , # [error ("Failed to interpolate the attribute file configured at `core.attributesFile`")] AttributesFileInterpolation (# [from] gix_config :: path :: interpolate :: Error) , } }
    };
}

attribute_stack!()