macro_rules! deps {
    () => {
        ThroughputOnDrop!();
        Id!();
        NestedProgress!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : NestedProgress > NestedProgress for ThroughputOnDrop < T > { type SubProgress = ThroughputOnDrop < T :: SubProgress > ; fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress { ThroughputOnDrop :: new (self . 0 . add_child (name)) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress { ThroughputOnDrop :: new (self . 0 . add_child_with_id (name , id)) } }
    };
}

impl_173!();