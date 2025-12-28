macro_rules! deps {
    () => {
        AttributesDigest!();
        Driver!();
    };
}

macro_rules! Configuration {
    () => {
        deps!();
        pub (crate) struct Configuration < 'a > { pub (crate) driver : Option < & 'a Driver > , # [doc = " What attributes say about CRLF handling."] pub (crate) _attr_digest : Option < eol :: AttributesDigest > , # [doc = " The final digest that includes configuration values"] pub (crate) digest : eol :: AttributesDigest , pub (crate) encoding : Option < & 'static encoding_rs :: Encoding > , # [doc = " Whether or not to apply the `ident` filter"] pub (crate) apply_ident_filter : bool , }
    };
}

Configuration!();