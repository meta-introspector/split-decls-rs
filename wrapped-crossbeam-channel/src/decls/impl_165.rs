macro_rules! deps {
    () => {
        Operation!();
        Selected!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl From < usize > for Selected { # [inline] fn from (val : usize) -> Self { match val { 0 => Self :: Waiting , 1 => Self :: Aborted , 2 => Self :: Disconnected , oper => Self :: Operation (Operation (oper)) , } } }
    };
}

impl_165!()