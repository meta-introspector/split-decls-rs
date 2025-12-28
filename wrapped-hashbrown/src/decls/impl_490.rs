macro_rules! deps {
    () => {
        AbsentEntry!();
        HashTable!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl < 'a , T , A > AbsentEntry < 'a , T , A > where A : Allocator , { # [doc = " Converts the `AbsentEntry` into a mutable reference to the underlying"] # [doc = " table."] pub fn into_table (self) -> & 'a mut HashTable < T , A > { self . table } }
    };
}

impl_490!();