macro_rules! deps {
    () => {
        ParseRequestError!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < mime :: FromStrError > for ParseRequestError { fn from (e : mime :: FromStrError) -> Self { Self :: InvalidRequest (Box :: new (e)) } }
    };
}

impl_51!();