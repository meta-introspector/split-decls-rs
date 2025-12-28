macro_rules! deps {
    () => {
        MultipartOptions!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl MultipartOptions { # [doc = " Set maximum file size."] # [must_use] pub fn max_file_size (self , size : usize) -> Self { MultipartOptions { max_file_size : Some (size) , .. self } } # [doc = " Set maximum number of files."] # [must_use] pub fn max_num_files (self , n : usize) -> Self { MultipartOptions { max_num_files : Some (n) , .. self } } }
    };
}

impl_624!()