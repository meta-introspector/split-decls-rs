macro_rules! deps {
    () => {
        Format!();
        Outcome!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'a > Outcome < 'a > { # [doc = " Turn this outcome into a structure that can display itself in the typical `git describe` format."] pub fn into_format (self , hex_len : usize) -> Format < 'a > { Format { name : self . name , id : self . id , hex_len , depth : self . depth , long : false , dirty_suffix : None , } } }
    };
}

impl_1!();