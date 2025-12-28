macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! prepare {
    () => {
        deps!();
        # [doc = ""] pub mod prepare { # [doc = " The error returned by [`prepare_fetch()`][super::Connection::prepare_fetch()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot perform a meaningful fetch operation without any configured ref-specs")] MissingRefSpecs , # [error (transparent)] RefMap (# [from] crate :: remote :: ref_map :: Error) , } impl gix_protocol :: transport :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: RefMap (err) => err . is_spurious () , _ => false , } } } }
    };
}

prepare!()