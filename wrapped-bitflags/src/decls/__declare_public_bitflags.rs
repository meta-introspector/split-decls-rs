macro_rules! deps {
    () => {
        PublicFlags!();
    };
}

macro_rules! __declare_public_bitflags {
    () => {
        deps!();
        # [doc = " Declare the user-facing bitflags struct."] # [doc = ""] # [doc = " This type is guaranteed to be a newtype with a `bitflags`-facing type as its single field."] # [macro_export] # [doc (hidden)] macro_rules ! __declare_public_bitflags { ($ (# [$ outer : meta]) * $ vis : vis struct $ PublicBitFlags : ident) => { $ (# [$ outer]) * $ vis struct $ PublicBitFlags (<$ PublicBitFlags as $ crate :: __private :: PublicFlags >:: Internal) ; } ; }
    };
}

__declare_public_bitflags!()