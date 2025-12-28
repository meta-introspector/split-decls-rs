macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! to_object {
    () => {
        deps!();
        # [doc = ""] pub mod to_object { use std :: path :: PathBuf ; use crate :: file ; # [doc = " The error returned by [`file::ReferenceExt::follow_to_object_packed()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not follow a single level of a symbolic reference")] Follow (# [from] file :: find :: existing :: Error) , # [error ("Aborting due to reference cycle with first seen path being {start_absolute:?}")] Cycle { start_absolute : PathBuf } , # [error ("Refusing to follow more than {max_depth} levels of indirection")] DepthLimitExceeded { max_depth : usize } , } }
    };
}

to_object!()