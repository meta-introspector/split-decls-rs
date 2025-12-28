macro_rules! deps {
    () => {
        StringInner!();
        LenType!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > Serialize for StringInner < LenT , S > { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { serializer . serialize_str (self) } }
    };
}

impl_354!()