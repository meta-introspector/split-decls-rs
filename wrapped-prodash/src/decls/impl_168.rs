macro_rules! deps {
    () => {
        NestedProgress!();
        DoOrDiscard!();
        Id!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T > NestedProgress for DoOrDiscard < T > where T : NestedProgress , { type SubProgress = DoOrDiscard < T :: SubProgress > ; fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress { DoOrDiscard (self . 0 . add_child (name)) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress { DoOrDiscard (self . 0 . add_child_with_id (name , id)) } }
    };
}

impl_168!()