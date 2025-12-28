macro_rules! NumberDeserializer {
    () => {
        # [cfg (feature = "arbitrary_precision")] pub (crate) struct NumberDeserializer { pub number : Option < String > , }
    };
}

NumberDeserializer!();