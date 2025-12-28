macro_rules! deps {
    () => {
        TreeWalkResult!();
    };
}

macro_rules! impl_816 {
    () => {
        deps!();
        impl Into < i32 > for TreeWalkResult { fn into (self) -> i32 { self as i32 } }
    };
}

impl_816!()