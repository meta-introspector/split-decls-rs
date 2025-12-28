macro_rules! deps {
    () => {
        Operation!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Operation { # [doc = " Creates an operation identifier from a mutable reference."] # [doc = ""] # [doc = " This function essentially just turns the address of the reference into a number. The"] # [doc = " reference should point to a variable that is specific to the thread and the operation,"] # [doc = " and is alive for the entire duration of select or blocking operation."] # [inline] pub fn hook < T > (r : & mut T) -> Self { let val = r as * mut T as usize ; assert ! (val > 2) ; Self (val) } }
    };
}

impl_163!()