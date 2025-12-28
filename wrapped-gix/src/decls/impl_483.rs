macro_rules! deps {
    () => {
        PathCursor!();
        Path!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl PathCursor < '_ > { fn at (& mut self , component : & str) -> & Path { self . 0 . push (component) ; self . 0 . as_path () } }
    };
}

impl_483!();