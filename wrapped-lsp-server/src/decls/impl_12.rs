macro_rules! deps {
    () => {
        Response!();
        Message!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl From < Response > for Message { fn from (response : Response) -> Message { Message :: Response (response) } }
    };
}

impl_12!()