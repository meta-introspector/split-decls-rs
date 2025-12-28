macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! update {
    () => {
        deps!();
        # [doc = ""] pub mod update { use bstr :: BString ; # [doc = " The error returned by [File::update()](crate::File::update)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The 'update' field of submodule '{submodule}' tried to set command '{actual}' to be shared")] CommandForbiddenInModulesConfiguration { submodule : BString , actual : BString } , # [error ("The 'update' field of submodule '{submodule}' was invalid: '{actual}'")] Invalid { submodule : BString , actual : BString } , } }
    };
}

update!();