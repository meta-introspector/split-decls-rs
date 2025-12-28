macro_rules! deps {
    () => {
        SymmetricKey!();
        V4!();
        Error!();
        FormatAsPaserk!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl FormatAsPaserk for SymmetricKey < V4 > { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str ("k4.local.") ? ; write . write_str (& encode_b64 (self . as_bytes ()) . map_err (| _ | core :: fmt :: Error) ?) } }
    };
}

impl_51!()