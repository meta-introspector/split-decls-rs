macro_rules! deps {
    () => {
        Url!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [doc = " Modification"] impl Url { # [doc = " Set the given `user`, or unset it with `None`. Return the previous value."] pub fn set_user (& mut self , user : Option < String >) -> Option < String > { let prev = self . user . take () ; self . user = user ; prev } # [doc = " Set the given `password`, or unset it with `None`. Return the previous value."] pub fn set_password (& mut self , password : Option < String >) -> Option < String > { let prev = self . password . take () ; self . password = password ; prev } }
    };
}

impl_49!();