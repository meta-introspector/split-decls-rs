macro_rules! impl_set_shareable {
    () => {
        macro_rules ! impl_set_shareable { () => { # [doc = " Indicate that an object type's field is allowed to be resolved by"] # [doc = " multiple subgraphs"] # [inline] pub fn shareable (self) -> Self { Self { shareable : true , .. self } } } ; }
    };
}

impl_set_shareable!();