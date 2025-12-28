macro_rules! deps {
    () => {
        Id!();
        Discard!();
        NestedProgress!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl NestedProgress for Discard { type SubProgress = Self ; fn add_child (& mut self , _name : impl Into < String >) -> Self { Discard } fn add_child_with_id (& mut self , _name : impl Into < String > , _id : Id) -> Self { Discard } }
    };
}

impl_158!()