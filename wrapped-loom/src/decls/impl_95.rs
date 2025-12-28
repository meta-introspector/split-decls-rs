macro_rules! deps {
    () => {
        Operation!();
        Action!();
        Ref!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Operation { pub (super) fn object (& self) -> Ref { self . obj } pub (super) fn action (& self) -> Action { self . action } pub (super) fn location (& self) -> Location { self . location } }
    };
}

impl_95!();