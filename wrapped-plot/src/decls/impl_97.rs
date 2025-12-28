macro_rules! deps {
    () => {
        Properties!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Properties { # [doc = " Hides the key"] pub fn hide (& mut self) -> & mut Properties { self . hidden = true ; self } # [doc = " Shows the key"] # [doc = ""] # [doc = " **Note** The key is shown by default"] pub fn show (& mut self) -> & mut Properties { self . hidden = false ; self } }
    };
}

impl_97!()