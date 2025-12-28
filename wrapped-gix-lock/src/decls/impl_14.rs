macro_rules! deps {
    () => {
        Marker!();
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Marker { # [doc = " Commit the changes written to the previously open file and overwrite the original file atomically, returning the resource path"] # [doc = " on success."] # [doc = ""] # [doc = " This fails for markers which weren't created with [`File::close()`]"] pub fn commit (mut self) -> Result < PathBuf , Error < Self > > { if ! self . created_from_file { return Err (Error { error : std :: io :: Error :: other ("refusing to commit marker that was never opened") , instance : self , }) ; } let resource_path = self . resource_path () ; match self . inner . persist (& resource_path) { Ok (_) => Ok (resource_path) , Err (err) => Err (Error { error : err . error , instance : { self . inner = err . handle ; self } , }) , } } }
    };
}

impl_14!();