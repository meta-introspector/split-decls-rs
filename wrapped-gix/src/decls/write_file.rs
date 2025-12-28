macro_rules! deps {
    () => {
        Error!();
        Path!();
    };
}

macro_rules! write_file {
    () => {
        deps!();
        fn write_file (data : & [u8] , path : & Path) -> Result < () , Error > { let mut file = OpenOptions :: new () . write (true) . create (true) . truncate (true) . append (false) . open (path) . map_err (| e | Error :: IoOpen { source : e , path : path . to_owned () , }) ? ; file . write_all (data) . map_err (| e | Error :: IoWrite { source : e , path : path . to_owned () , }) }
    };
}

write_file!();