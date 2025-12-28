macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
        FormatAsPaserk!();
        V3!();
        Error!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [cfg (feature = "v3")] impl FormatAsPaserk for AsymmetricSecretKey < V3 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k3.secret.") ? ; write . write_str (& encode_b64 (& self . bytes) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_55!();