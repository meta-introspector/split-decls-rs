macro_rules! deps {
    () => {
        FormatAsPaserk!();
        V2!();
        AsymmetricPublicKey!();
        Error!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl FormatAsPaserk for AsymmetricPublicKey < V2 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k2.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_59!()