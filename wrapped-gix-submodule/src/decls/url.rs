macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! url {
    () => {
        deps!();
        # [doc = ""] pub mod url { use bstr :: BString ; # [doc = " The error returned by [File::url()](crate::File::url)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The url of submodule '{submodule}' could not be parsed")] Parse { submodule : BString , source : gix_url :: parse :: Error , } , # [error ("The submodule '{submodule}' was missing its 'url' field or it was empty")] Missing { submodule : BString } , } }
    };
}

url!();