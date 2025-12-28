macro_rules! Id {
    () => {
        # [derive (Debug , Clone)] # [doc = " PASERK IDs."] # [doc = ""] # [doc = " This operation calculates the unique ID for a given PASERK."] # [doc = ""] # [doc = " See: <https://github.com/paseto-standard/paserk/blob/master/operations/ID.md>"] pub struct Id { header : String , identifier : String , }
    };
}

Id!();