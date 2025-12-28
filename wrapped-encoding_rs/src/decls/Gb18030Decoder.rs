macro_rules! deps {
    () => {
        Gb18030Pending!();
    };
}

macro_rules! Gb18030Decoder {
    () => {
        deps!();
        pub struct Gb18030Decoder { first : Option < u8 > , second : Option < u8 > , third : Option < u8 > , pending : Gb18030Pending , pending_ascii : Option < u8 > , }
    };
}

Gb18030Decoder!();