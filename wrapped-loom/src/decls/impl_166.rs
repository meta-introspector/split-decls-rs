macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl Id { pub (crate) fn new (execution_id : execution :: Id , id : usize) -> Id { Id { execution_id , id } } pub (crate) fn as_usize (self) -> usize { self . id } }
    };
}

impl_166!();