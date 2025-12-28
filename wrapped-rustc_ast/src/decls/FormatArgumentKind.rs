macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! FormatArgumentKind {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum FormatArgumentKind { # [doc = " `format_args(…, arg)`"] Normal , # [doc = " `format_args(…, arg = 1)`"] Named (Ident) , # [doc = " `format_args(\"… {arg} …\")`"] Captured (Ident) , }
    };
}

FormatArgumentKind!();