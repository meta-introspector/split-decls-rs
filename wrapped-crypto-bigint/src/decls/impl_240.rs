macro_rules! deps {
    () => {
        Odd!();
        Uint!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl Odd < UintRef > { # [doc = " Construct an [`Odd<Uint<T>>`] from the unsigned integer value,"] # [doc = " truncating the upper bits if the value is too large to be"] # [doc = " represented."] pub const fn to_uint_resize < const T : usize > (& self) -> Odd < Uint < T > > { Odd (self . 0 . to_uint_resize ()) } }
    };
}

impl_240!()