macro_rules! deps {
    () => {
        FormatAsPaserk!();
        Error!();
        AsymmetricPublicKey!();
        V4!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl FormatAsPaserk for AsymmetricPublicKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_63!()