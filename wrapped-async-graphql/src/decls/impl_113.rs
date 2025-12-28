macro_rules! deps {
    () => {
        Response!();
        BatchResponse!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl From < Response > for BatchResponse { fn from (response : Response) -> Self { Self :: Single (response) } }
    };
}

impl_113!();