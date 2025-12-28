macro_rules! deps {
    () => {
        ErrorImpl!();
        Serialize!();
        Result!();
        Serializer!();
        Error!();
    };
}

macro_rules! serialize {
    () => {
        deps!();
        # [doc = " Serialize the given type-erased serializable value."] # [doc = ""] # [doc = " This can be used to implement `serde::Serialize` for trait objects that have"] # [doc = " `erased_serde::Serialize` as a supertrait."] # [doc = ""] # [doc = " ```"] # [doc = " trait Event: erased_serde::Serialize {"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " impl<'a> serde::Serialize for dyn Event + 'a {"] # [doc = "     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>"] # [doc = "     where"] # [doc = "         S: serde::Serializer,"] # [doc = "     {"] # [doc = "         erased_serde::serialize(self, serializer)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Since this is reasonably common, the `serialize_trait_object!` macro"] # [doc = " generates such a Serialize impl."] # [doc = ""] # [doc = " ```"] # [doc = " use erased_serde::serialize_trait_object;"] # [doc = " #"] # [doc = " # trait Event: erased_serde::Serialize {}"] # [doc = ""] # [doc = " serialize_trait_object!(Event);"] # [doc = " ```"] pub fn serialize < T , S > (value : & T , serializer : S) -> Result < S :: Ok , S :: Error > where T : ? Sized + Serialize , S : serde :: Serializer , { let mut erased = erase :: Serializer :: new (serializer) ; match value . do_erased_serialize (& mut erased) { Ok (()) | Err (ShortCircuit) => { } Err (ErrorImpl :: Custom (msg)) => return Err (serde :: ser :: Error :: custom (msg)) , } match erased { erase :: Serializer :: Complete (ok) => Ok (ok) , erase :: Serializer :: Error (err) => Err (err) , _ => unreachable ! () , } }
    };
}

serialize!();