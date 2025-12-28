macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: PostBody (err) => err . is_spurious () , # [cfg (any (feature = "http-client-reqwest" , feature = "http-client-curl"))] Error :: InitHttpClient { source } => { # [cfg (feature = "http-client-curl")] if let Some (err) = source . downcast_ref :: < crate :: client :: blocking_io :: http :: curl :: Error > () { return err . is_spurious () ; } # [cfg (feature = "http-client-reqwest")] if let Some (err) = source . downcast_ref :: < crate :: client :: blocking_io :: http :: reqwest :: remote :: Error > () { return err . is_spurious () ; } false } _ => false , } } }
    };
}

impl_79!();