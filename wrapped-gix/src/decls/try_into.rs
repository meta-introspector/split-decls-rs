macro_rules! deps {
    () => {
        Error!();
        Object!();
        Kind!();
    };
}

macro_rules! try_into {
    () => {
        deps!();
        # [doc = ""] pub mod try_into { # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] # [error ("Object named {id} was supposed to be of kind {expected}, but was kind {actual}.")] pub struct Error { pub actual : gix_object :: Kind , pub expected : gix_object :: Kind , pub id : gix_hash :: ObjectId , } }
    };
}

try_into!()