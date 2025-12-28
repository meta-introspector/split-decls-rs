macro_rules! deps {
    () => {
        AnsiByteStrings!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > AnsiByteStrings < 'a > { # [doc = " Write `AnsiByteStrings` to an `io::Write`.  This writes the minimal"] # [doc = " escape sequences for the associated `Style`s around each set of"] # [doc = " bytes."] pub fn write_to < W : std :: io :: Write > (& self , w : & mut W) -> std :: io :: Result < () > { let w : & mut dyn std :: io :: Write = w ; self . write_to_any (w) } }
    };
}

impl_48!();