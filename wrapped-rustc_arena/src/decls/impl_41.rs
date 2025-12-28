macro_rules! deps {
    () => {
        SmallDroppable!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Drop for SmallDroppable { fn drop (& mut self) { DROP_COUNTER . with (| c | c . set (c . get () + 1)) ; } }
    };
}

impl_41!()