macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [doc = " The constant-time alternative is available at [`NonIdentity::new()`]."] impl < C > TryFrom < ProjectivePoint < C > > for NonIdentity < ProjectivePoint < C > > where C : PrimeCurveParams , { type Error = Error ; fn try_from (point : ProjectivePoint < C >) -> Result < Self > { NonIdentity :: new (point) . into_option () . ok_or (Error) } }
    };
}

impl_82!();