macro_rules! deps {
    () => {
        ConsumeBinaryHunkDelegate!();
        ConsumeBinaryHunk!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'a , D > ConsumeBinaryHunk < 'a , D > where D : ConsumeBinaryHunkDelegate , { # [doc = " Create a new instance that writes stringified hunks to `delegate`, which uses `newline` to separate header and hunk,"] # [doc = " as well as hunk lines that don't naturally end in a newline."] pub fn new (delegate : D , newline : & 'a str) -> ConsumeBinaryHunk < 'a , D > { ConsumeBinaryHunk { newline , delegate , header_buf : String :: new () , hunk_buf : Vec :: with_capacity (128) , } } }
    };
}

impl_140!()