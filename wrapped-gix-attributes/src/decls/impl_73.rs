macro_rules! deps {
    () => {
        StateRef!();
        Iter!();
        Error!();
        AssignmentRef!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a > Iter < 'a > { # [doc = " Create a new instance to parse attribute assignments from `input`."] pub fn new (input : & 'a BStr) -> Self { Iter { attrs : input . fields () } } fn parse_attr (& self , attr : & 'a [u8]) -> Result < AssignmentRef < 'a > , name :: Error > { let mut tokens = attr . splitn (2 , | b | * b == b'=') ; let attr = tokens . next () . expect ("attr itself") . as_bstr () ; let possibly_value = tokens . next () ; let (attr , state) = if attr . first () == Some (& b'-') { (& attr [1 ..] , StateRef :: Unset) } else if attr . first () == Some (& b'!') { (& attr [1 ..] , StateRef :: Unspecified) } else { (attr , possibly_value . map_or (StateRef :: Set , StateRef :: from_bytes)) } ; Ok (AssignmentRef :: new (check_attr (attr) ? , state)) } }
    };
}

impl_73!();