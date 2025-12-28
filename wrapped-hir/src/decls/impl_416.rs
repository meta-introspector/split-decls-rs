macro_rules! deps {
    () => {
        Union!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl HasCrate for Union { fn krate (& self , db : & dyn HirDatabase) -> Crate { self . module (db) . krate () } }
    };
}

impl_416!();