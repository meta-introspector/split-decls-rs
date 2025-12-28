macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! save {
    () => {
        deps!();
        pub fn save < D , P > (data : & D , path : & P) -> Result < () > where D : Serialize , P : AsRef < Path > , { let buf = serde_json :: to_string (& data) . map_err (| inner | Error :: SerdeError { path : path . as_ref () . to_owned () , inner , }) ? ; save_string (& buf , path) }
    };
}

save!()