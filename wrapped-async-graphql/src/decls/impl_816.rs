macro_rules! deps {
    () => {
        Result!();
        UploadValue!();
        Upload!();
        Context!();
    };
}

macro_rules! impl_816 {
    () => {
        deps!();
        impl Upload { # [doc = " Get the upload value."] pub fn value (& self , ctx : & Context < '_ >) -> std :: io :: Result < UploadValue > { ctx . query_env . uploads [self . 0] . try_clone () } }
    };
}

impl_816!();