macro_rules! ErrorConvert {
    () => {
        # [doc = " Equivalent From implementation to avoid orphan rules in bits parsers"] pub trait ErrorConvert < E > { # [doc = " Transform to another error type"] fn convert (self) -> E ; }
    };
}

ErrorConvert!()