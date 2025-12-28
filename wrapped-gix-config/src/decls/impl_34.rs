macro_rules! deps {
    () => {
        Options!();
        Error!();
        Metadata!();
        File!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl File < 'static > { # [doc = " Instantiate a new fully-owned `File` from given `input` (later reused as buffer when resolving includes),"] # [doc = " associating each section and their values with `meta`-data, while respecting `options`, and"] # [doc = " following includes as configured there."] pub fn from_bytes_owned (input_and_buf : & mut Vec < u8 > , meta : impl Into < OwnShared < Metadata > > , options : Options < '_ > ,) -> Result < Self , Error > { let mut config = Self :: from_parse_events_no_includes (parse :: Events :: from_bytes_owned (input_and_buf , options . to_event_filter ()) . map_err (Error :: from) ? , meta ,) ; includes :: resolve (& mut config , input_and_buf , options) . map_err (Error :: from) ? ; Ok (config) } }
    };
}

impl_34!()