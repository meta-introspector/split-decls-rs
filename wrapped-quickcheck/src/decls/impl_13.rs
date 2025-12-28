macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < A : Arbitrary , B : Arbitrary > Arbitrary for Result < A , B > { fn arbitrary (g : & mut Gen) -> Result < A , B > { if g . random () { Ok (Arbitrary :: arbitrary (g)) } else { Err (Arbitrary :: arbitrary (g)) } } fn shrink (& self) -> Box < dyn Iterator < Item = Result < A , B > > > { match * self { Ok (ref x) => { let xs = x . shrink () ; let tagged = xs . map (Ok) ; Box :: new (tagged) } Err (ref x) => { let xs = x . shrink () ; let tagged = xs . map (Err) ; Box :: new (tagged) } } } }
    };
}

impl_13!()