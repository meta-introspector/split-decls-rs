macro_rules! Name {
    () => {
        # [doc = " `Name` is a wrapper around string, which is used in hir for both references"] # [doc = " and declarations. In theory, names should also carry hygiene info, but we are"] # [doc = " not there yet!"] # [doc = ""] # [doc = " Note that the rawness (`r#`) of names is not preserved. Names are always stored without a `r#` prefix."] # [doc = " This is because we want to show (in completions etc.) names as raw depending on the needs"] # [doc = " of the current crate, for example if it is edition 2021 complete `gen` even if the defining"] # [doc = " crate is in edition 2024 and wrote `r#gen`, and the opposite holds as well."] # [derive (Clone , PartialEq , Eq , Hash)] pub struct Name { symbol : Symbol , ctx : () , }
    };
}

Name!()