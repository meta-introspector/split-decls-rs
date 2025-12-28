macro_rules! deps {
    () => {
        Error!();
        Validate!();
    };
}

macro_rules! set_value {
    () => {
        deps!();
        # [doc = ""] pub mod set_value { # [doc = " The error produced when calling [`SnapshotMut::set(_subsection)?_value()`][crate::config::SnapshotMut::set_value()]"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] SetRaw (# [from] gix_config :: file :: set_raw_value :: Error) , # [error (transparent)] Validate (# [from] crate :: config :: tree :: key :: validate :: Error) , # [error ("The key needs a subsection parameter to be valid.")] SubSectionRequired , # [error ("The key must not be used with a subsection")] SubSectionForbidden , } }
    };
}

set_value!()