macro_rules! deps {
    () => {
        Operation!();
        Selected!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl From < Selected > for usize { # [inline] fn from (val : Selected) -> Self { match val { Selected :: Waiting => 0 , Selected :: Aborted => 1 , Selected :: Disconnected => 2 , Selected :: Operation (Operation (val)) => val , } } }
    };
}

impl_166!()