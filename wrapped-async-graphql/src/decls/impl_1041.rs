macro_rules! deps {
    () => {
        MetaTypeId!();
        Result!();
        Scalar!();
        Union!();
        InputObject!();
        Object!();
        Interface!();
    };
}

macro_rules! impl_1041 {
    () => {
        deps!();
        impl Display for MetaTypeId { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { MetaTypeId :: Scalar => "Scalar" , MetaTypeId :: Object => "Object" , MetaTypeId :: Interface => "Interface" , MetaTypeId :: Union => "Union" , MetaTypeId :: Enum => "Enum" , MetaTypeId :: InputObject => "InputObject" , }) } }
    };
}

impl_1041!()