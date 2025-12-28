macro_rules! deps {
    () => {
        Offset!();
        DateTime!();
        TimeZone!();
        NaiveDateTime!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        # [cfg (all (feature = "arbitrary" , feature = "std"))] impl < 'a , Tz > arbitrary :: Arbitrary < 'a > for DateTime < Tz > where Tz : TimeZone , < Tz as TimeZone > :: Offset : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < DateTime < Tz > > { let datetime = NaiveDateTime :: arbitrary (u) ? ; let offset = < Tz as TimeZone > :: Offset :: arbitrary (u) ? ; Ok (DateTime :: from_naive_utc_and_offset (datetime , offset)) } }
    };
}

impl_194!()