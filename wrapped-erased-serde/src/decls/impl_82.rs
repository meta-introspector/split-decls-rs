macro_rules! deps {
    () => {
        Serializer!();
        ErrorImpl!();
        Error!();
        MakeSerializer!();
        Result!();
        Serialize!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T > Serialize for T where T : ? Sized + serde :: Serialize , { fn erased_serialize (& self , serializer : & mut dyn Serializer) -> Result < () , Error > { match self . do_erased_serialize (serializer) { Ok (()) => Ok (()) , Err (ShortCircuit) => Err (serde :: ser :: Error :: custom (serializer . erased_display_error ())) , Err (ErrorImpl :: Custom (msg)) => Err (serde :: ser :: Error :: custom (msg)) , } } fn do_erased_serialize (& self , serializer : & mut dyn Serializer) -> Result < () , ErrorImpl > { self . serialize (MakeSerializer (serializer)) } }
    };
}

impl_82!();