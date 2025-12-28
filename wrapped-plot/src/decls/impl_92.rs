macro_rules! deps {
    () => {
        Properties!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Properties { # [doc = " Hides the gridlines"] # [doc = ""] # [doc = " **Note** Both `Major` and `Minor` gridlines are hidden by default"] pub fn hide (& mut self) -> & mut Properties { self . hidden = true ; self } # [doc = " Shows the gridlines"] pub fn show (& mut self) -> & mut Properties { self . hidden = false ; self } }
    };
}

impl_92!()