macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! impl_set_deprecation {
    () => {
        deps!();
        macro_rules ! impl_set_deprecation { () => { # [doc = " Set the description"] # [inline] pub fn deprecation (self , reason : Option <& str >) -> Self { Self { deprecation : Deprecation :: Deprecated { reason : reason . map (Into :: into) , } , .. self } } } ; }
    };
}

impl_set_deprecation!()