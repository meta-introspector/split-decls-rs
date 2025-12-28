macro_rules! impl_set_external {
    () => {
        macro_rules ! impl_set_external { () => { # [doc = " Mark a field as owned by another service. This allows service A to use"] # [doc = " fields from service B while also knowing at runtime the types of that"] # [doc = " field."] # [inline] pub fn external (self) -> Self { Self { external : true , .. self } } } ; }
    };
}

impl_set_external!();