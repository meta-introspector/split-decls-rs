macro_rules! deps {
    () => {
        TimeZone!();
        Offset!();
        Date!();
        NaiveDate!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (all (feature = "arbitrary" , feature = "std"))] impl < 'a , Tz > arbitrary :: Arbitrary < 'a > for Date < Tz > where Tz : TimeZone , < Tz as TimeZone > :: Offset : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Date < Tz > > { let date = NaiveDate :: arbitrary (u) ? ; let offset = < Tz as TimeZone > :: Offset :: arbitrary (u) ? ; Ok (Date :: from_utc (date , offset)) } }
    };
}

impl_56!();