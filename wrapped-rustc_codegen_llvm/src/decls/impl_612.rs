macro_rules! impl_612 {
    () => {
        impl PartialEq for Value { fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } }
    };
}

impl_612!()