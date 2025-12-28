macro_rules! deps {
    () => {
        RefCnt!();
        ArcSwapAny!();
        Strategy!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < T , S > Serialize for ArcSwapAny < T , S > where T : RefCnt + Serialize , S : Strategy < T > , { fn serialize < Ser : Serializer > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > { self . load () . serialize (serializer) } }
    };
}

impl_100!()