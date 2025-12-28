macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! cp {
    () => {
        deps!();
        pub fn cp (from : & Path , to : & Path) -> Result < () > { fs :: copy (from , to) . map_err (| inner | Error :: CopyError { inner , from : from . to_owned () , to : to . to_owned () , }) ? ; Ok (()) }
    };
}

cp!();