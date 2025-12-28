macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'a > From < Sp < & 'a str > > for Sp < String > { fn from (sp : Sp < & 'a str >) -> Self { Sp :: new (sp . val . into () , sp . span) } }
    };
}

impl_97!()