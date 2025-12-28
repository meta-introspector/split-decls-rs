macro_rules! deps {
    () => {
        Unit!();
        ByteSize!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl str :: FromStr for ByteSize { type Err = String ; fn from_str (value : & str) -> Result < Self , Self :: Err > { if let Ok (v) = value . parse :: < u64 > () { return Ok (Self (v)) ; } let number = take_while (value , | c | c . is_ascii_digit () || c == '.') ; match number . parse :: < f64 > () { Ok (v) => { let suffix = skip_while (& value [number . len () ..] , char :: is_whitespace) ; match suffix . parse :: < Unit > () { Ok (u) => Ok (Self ((v * u) as u64)) , Err (error) => Err (format ! ("couldn't parse {suffix:?} into a known SI unit, {error}")) , } } Err (error) => Err (format ! ("couldn't parse {value:?} into a ByteSize, {error}")) , } } }
    };
}

impl_12!()