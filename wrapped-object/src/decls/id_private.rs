macro_rules! id_private {
    () => {
        mod id_private { pub trait IdPrivate { fn new (id : usize) -> Self ; } }
    };
}

id_private!()