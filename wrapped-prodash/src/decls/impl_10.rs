macro_rules! deps {
    () => {
        NestedProgress!();
        Item!();
        Id!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl crate :: NestedProgress for Item { type SubProgress = Item ; fn add_child (& mut self , name : impl Into < String >) -> Self { Item :: add_child (self , name) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self { Item :: add_child_with_id (self , name , id) } }
    };
}

impl_10!()