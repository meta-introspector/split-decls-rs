macro_rules! deps {
    () => {
        Error!();
        Header!();
    };
}

macro_rules! ext {
    () => {
        deps!();
        mod ext { use crate :: find ; # [doc = " An extension trait with convenience functions."] pub trait HeaderExt : super :: Header { # [doc = " Like [`try_header(…)`][super::Header::try_header()], but flattens the `Result<Option<_>>` into a single `Result` making a non-existing object an error."] fn header (& self , id : impl AsRef < gix_hash :: oid >) -> Result < find :: Header , gix_object :: find :: existing :: Error > { let id = id . as_ref () ; self . try_header (id) . map_err (gix_object :: find :: existing :: Error :: Find) ? . ok_or_else (| | gix_object :: find :: existing :: Error :: NotFound { oid : id . to_owned () }) } } impl < T : super :: Header > HeaderExt for T { } }
    };
}

ext!();