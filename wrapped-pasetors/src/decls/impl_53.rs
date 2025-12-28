macro_rules! deps {
    () => {
        FormatAsPaserk!();
        AsymmetricSecretKey!();
        V2!();
        Error!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl FormatAsPaserk for AsymmetricSecretKey < V2 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k2.secret.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_53!();