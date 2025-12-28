macro_rules! deps {
    () => {
        Options!();
        Platform!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl Platform < '_ , '_ > { # [doc = " Adjust diff options with `change_opts`."] pub fn options (& mut self , change_opts : impl FnOnce (& mut crate :: diff :: Options)) -> & mut Self { change_opts (& mut self . options) ; self } }
    };
}

impl_215!();