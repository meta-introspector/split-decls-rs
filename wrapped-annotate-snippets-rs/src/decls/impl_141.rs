macro_rules! deps {
    () => {
        Origin!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a > Origin < 'a > { # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Text passed to this function is considered \"untrusted input\", as such"] # [doc = " all text is passed through a normalization function. Pre-styled text is"] # [doc = " not allowed to be passed to this function."] # [doc = ""] # [doc = " </div>"] pub fn path (path : impl Into < Cow < 'a , str > >) -> Self { Self { path : path . into () , line : None , char_column : None , } } # [doc = " Set the default line number to display"] pub fn line (mut self , line : usize) -> Self { self . line = Some (line) ; self } # [doc = " Set the default column to display"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " `char_column` is only be respected if [`Origin::line`] is also set."] # [doc = ""] # [doc = " </div>"] pub fn char_column (mut self , char_column : usize) -> Self { self . char_column = Some (char_column) ; self } }
    };
}

impl_141!()