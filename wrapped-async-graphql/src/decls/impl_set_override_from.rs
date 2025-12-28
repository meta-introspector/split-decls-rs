macro_rules! impl_set_override_from {
    () => {
        macro_rules ! impl_set_override_from { () => { # [doc = " Indicate that an object type's field is allowed to be resolved by"] # [doc = " multiple subgraphs"] # [inline] pub fn override_from (self , name : impl Into < String >) -> Self { Self { override_from : Some (name . into ()) , .. self } } } ; }
    };
}

impl_set_override_from!();