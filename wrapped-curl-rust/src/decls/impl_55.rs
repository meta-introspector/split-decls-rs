macro_rules! deps {
    () => {
        Transfer!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'easy , 'data > Drop for Transfer < 'easy , 'data > { fn drop (& mut self) { assert ! (self . easy . inner . get_ref () . borrowed . get () . is_null ()) ; } }
    };
}

impl_55!();