macro_rules! impl_set_extends {
    () => {
        macro_rules ! impl_set_extends { () => { # [doc = " Indicates that an object or interface definition is an extension of another"] # [doc = " definition of that same type."] # [inline] pub fn extends (self) -> Self { Self { extends : true , .. self } } } ; }
    };
}

impl_set_extends!()