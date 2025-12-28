macro_rules! deps {
    () => {
        Error!();
        BlockBuffer!();
        BufferKind!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < BS : ArraySize , K : BufferKind > fmt :: Debug for BlockBuffer < BS , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct (K :: NAME) . field ("pos" , & self . get_pos ()) . field ("block_size" , & BS :: USIZE) . field ("data" , & self . get_data ()) . finish () } }
    };
}

impl_15!()