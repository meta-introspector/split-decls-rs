macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        V3!();
        Error!();
        FormatAsPaserk!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (feature = "v3")] impl FormatAsPaserk for AsymmetricPublicKey < V3 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k3.public.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_61!();