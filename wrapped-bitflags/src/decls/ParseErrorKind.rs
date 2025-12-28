macro_rules! ParseErrorKind {
    () => {
        # [derive (Debug)] # [allow (clippy :: enum_variant_names)] enum ParseErrorKind { EmptyFlag , InvalidNamedFlag { # [cfg (not (feature = "std"))] got : () , # [cfg (feature = "std")] got : String , } , InvalidHexFlag { # [cfg (not (feature = "std"))] got : () , # [cfg (feature = "std")] got : String , } , }
    };
}

ParseErrorKind!();