macro_rules! deps {
    () => {
        File!();
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl File { # [doc = " Commit the changes written to this lock file and overwrite the original file atomically, returning the resource path"] # [doc = " and an open file handle on success."] pub fn commit (mut self) -> Result < (PathBuf , Option < std :: fs :: File >) , Error < Self > > { let resource_path = self . resource_path () ; match self . inner . persist (& resource_path) { Ok (possibly_file) => Ok ((resource_path , possibly_file)) , Err (err) => Err (Error { error : err . error , instance : { self . inner = err . handle ; self } , }) , } } }
    };
}

impl_15!()