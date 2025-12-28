macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! branch {
    () => {
        deps!();
        # [doc = ""] pub mod branch { use bstr :: BString ; # [doc = " The error returned by [File::branch()](crate::File::branch)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error ("The value '{actual}' of the 'branch' field of submodule '{submodule}' couldn't be turned into a valid fetch refspec")] pub struct Error { pub submodule : BString , pub actual : BString , pub source : gix_refspec :: parse :: Error , } }
    };
}

branch!();