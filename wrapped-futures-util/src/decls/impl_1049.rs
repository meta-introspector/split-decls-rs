macro_rules! deps {
    () => {
        AllowStdIo!();
    };
}

macro_rules! impl_1049 {
    () => {
        deps!();
        impl < T > AllowStdIo < T > { # [doc = " Creates a new `AllowStdIo` from an existing IO object."] pub fn new (io : T) -> Self { Self (io) } # [doc = " Returns a reference to the contained IO object."] pub fn get_ref (& self) -> & T { & self . 0 } # [doc = " Returns a mutable reference to the contained IO object."] pub fn get_mut (& mut self) -> & mut T { & mut self . 0 } # [doc = " Consumes self and returns the contained IO object."] pub fn into_inner (self) -> T { self . 0 } }
    };
}

impl_1049!();