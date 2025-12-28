macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! load {
    () => {
        deps!();
        pub fn load < A , P > (path : & P) -> Result < A > where A : DeserializeOwned , P : AsRef < Path > + ? Sized , { let path = path . as_ref () ; let string = std :: fs :: read_to_string (path) . map_err (| inner | Error :: AccessError { inner , path : path . to_owned () , }) ? ; let result : A = serde_json :: from_str (string . as_str ()) . map_err (| inner | Error :: SerdeError { inner , path : path . to_owned () , }) ? ; Ok (result) }
    };
}

load!()