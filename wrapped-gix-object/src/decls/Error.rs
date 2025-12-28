macro_rules! Error {
    () => {
        # [doc = " The Error used in [`Kind::from_bytes()`]."] # [derive (Debug , Clone , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Unknown object kind: {kind:?}")] InvalidObjectKind { kind : bstr :: BString } , }
    };
}

Error!();