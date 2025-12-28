macro_rules! deps {
    () => {
        Notification!();
        Message!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl From < Notification > for Message { fn from (notification : Notification) -> Message { Message :: Notification (notification) } }
    };
}

impl_13!()