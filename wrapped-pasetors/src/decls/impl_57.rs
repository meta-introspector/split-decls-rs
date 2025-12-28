macro_rules! deps {
    () => {
        Error!();
        AsymmetricSecretKey!();
        FormatAsPaserk!();
        V4!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl FormatAsPaserk for AsymmetricSecretKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.secret.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_57!()