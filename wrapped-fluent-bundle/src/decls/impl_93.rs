macro_rules! deps {
    () => {
        FluentType!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl PartialEq for dyn FluentType + Send { fn eq (& self , other : & Self) -> bool { self . equals (other . as_any ()) } }
    };
}

impl_93!();