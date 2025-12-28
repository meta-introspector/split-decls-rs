macro_rules! impl_set_description {
    () => {
        macro_rules ! impl_set_description { () => { # [doc = " Set the description"] # [inline] pub fn description (self , description : impl Into < String >) -> Self { Self { description : Some (description . into ()) , .. self } } } ; }
    };
}

impl_set_description!();