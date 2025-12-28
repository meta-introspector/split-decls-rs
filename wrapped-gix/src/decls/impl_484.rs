macro_rules! deps {
    () => {
        NewDir!();
        Error!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl NewDir < '_ > { fn at (self , component : & str) -> Result < Self , Error > { self . 0 . push (component) ; create_dir (self . 0) ? ; Ok (self) } fn as_mut (& mut self) -> & mut PathBuf { self . 0 } }
    };
}

impl_484!();