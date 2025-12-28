macro_rules! deps {
    () => {
        ResourceDirectoryTable!();
        ResourceDirectory!();
        Result!();
    };
}

macro_rules! impl_720 {
    () => {
        deps!();
        impl < 'data > ResourceDirectory < 'data > { # [doc = " Construct from the data of the `.rsrc` section."] pub fn new (data : & 'data [u8]) -> Self { ResourceDirectory { data } } # [doc = " Parses the root resource directory."] pub fn root (& self) -> Result < ResourceDirectoryTable < 'data > > { ResourceDirectoryTable :: parse (self . data , 0) } }
    };
}

impl_720!()