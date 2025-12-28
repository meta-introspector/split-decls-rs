macro_rules! deps {
    () => {
        Error!();
        WantedRef!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl WantedRef { # [doc = " Parse a `WantedRef` from a `line` as received from the server."] pub fn from_line (line : & str) -> Result < WantedRef , Error > { match line . trim_end () . split_once (' ') { Some ((id , path)) => { let id = gix_hash :: ObjectId :: from_hex (id . as_bytes ()) . map_err (| _ | Error :: UnknownLineType { line : line . to_owned () }) ? ; Ok (WantedRef { id , path : path . into () }) } None => Err (Error :: UnknownLineType { line : line . to_owned () }) , } } }
    };
}

impl_24!()