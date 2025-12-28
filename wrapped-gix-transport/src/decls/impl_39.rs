macro_rules! deps {
    () => {
        ReadStdoutFailOnError!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl std :: io :: Read for ReadStdoutFailOnError { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let res = self . read . read (buf) ; self . swap_err_if_present_in_stderr (buf . len () , res) } }
    };
}

impl_39!()