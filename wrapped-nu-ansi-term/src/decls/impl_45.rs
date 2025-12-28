macro_rules! deps {
    () => {
        Style!();
        AnsiByteString!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > AnsiByteString < 'a > { # [doc = " Write an `AnsiByteString` to an `io::Write`.  This writes the escape"] # [doc = " sequences for the associated `Style` around the bytes."] pub fn write_to < W : std :: io :: Write > (& self , w : & mut W) -> std :: io :: Result < () > { let w : & mut dyn std :: io :: Write = w ; self . write_to_any (w) } }
    };
}

impl_45!()