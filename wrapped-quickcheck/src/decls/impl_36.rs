macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Arbitrary for OsString { fn arbitrary (g : & mut Gen) -> OsString { OsString :: from (String :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = OsString > > { let mystring : String = self . clone () . into_string () . unwrap () ; Box :: new (mystring . shrink () . map (OsString :: from)) } }
    };
}

impl_36!()