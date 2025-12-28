macro_rules! deps {
    () => {
        WriteTo!();
        Kind!();
        Write!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T > WriteTo for & T where T : WriteTo , { fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { < T as WriteTo > :: write_to (self , out) } fn kind (& self) -> Kind { < T as WriteTo > :: kind (self) } fn size (& self) -> u64 { < T as WriteTo > :: size (self) } }
    };
}

impl_171!();