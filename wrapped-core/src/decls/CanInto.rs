macro_rules! CanInto {
    () => {
        pub trait CanInto < T > : Sized { const QUERY : bool = false ; }
    };
}

CanInto!()