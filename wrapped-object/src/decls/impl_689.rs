macro_rules! deps {
    () => {
        ExportTarget!();
        ByteString!();
        Result!();
        Error!();
    };
}

macro_rules! impl_689 {
    () => {
        deps!();
        impl < 'a > Debug for ExportTarget < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: result :: Result < () , core :: fmt :: Error > { match self { ExportTarget :: Address (address) => write ! (f , "Address({:#x})" , address) , ExportTarget :: ForwardByOrdinal (library , ordinal) => write ! (f , "ForwardByOrdinal({:?}.#{})" , ByteString (library) , ordinal) , ExportTarget :: ForwardByName (library , name) => write ! (f , "ForwardByName({:?}.{:?})" , ByteString (library) , ByteString (name)) , } } }
    };
}

impl_689!()