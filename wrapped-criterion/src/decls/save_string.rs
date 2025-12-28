macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! save_string {
    () => {
        deps!();
        pub fn save_string < P > (data : & str , path : & P) -> Result < () > where P : AsRef < Path > , { use std :: io :: Write ; File :: create (path) . and_then (| mut f | f . write_all (data . as_bytes ())) . map_err (| inner | Error :: AccessError { inner , path : path . as_ref () . to_owned () , }) ? ; Ok (()) }
    };
}

save_string!();