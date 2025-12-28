macro_rules! deps {
    () => {
        Path!();
        Error!();
    };
}

macro_rules! create_dir {
    () => {
        deps!();
        fn create_dir (p : & Path) -> Result < () , Error > { fs :: create_dir_all (p) . map_err (| e | Error :: CreateDirectory { source : e , path : p . to_owned () , }) }
    };
}

create_dir!()