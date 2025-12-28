macro_rules! deps {
    () => {
        Message!();
        Request!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < Request > for Message { fn from (request : Request) -> Message { Message :: Request (request) } }
    };
}

impl_11!()