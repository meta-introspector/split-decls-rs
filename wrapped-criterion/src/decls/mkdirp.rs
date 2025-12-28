macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! mkdirp {
    () => {
        deps!();
        pub fn mkdirp < P > (path : & P) -> Result < () > where P : AsRef < Path > , { fs :: create_dir_all (path . as_ref ()) . map_err (| inner | Error :: AccessError { inner , path : path . as_ref () . to_owned () , }) }
    };
}

mkdirp!();