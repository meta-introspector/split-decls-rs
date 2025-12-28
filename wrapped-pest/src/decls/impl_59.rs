macro_rules! deps {
    () => {
        Pairs!();
        Error!();
        RuleType!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (feature = "pretty-print")] impl < R : RuleType > :: serde :: Serialize for Pairs < '_ , R > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { let start = self . pos (self . start) ; let end = self . pos (self . end - 1) ; let pairs = self . clone () . collect :: < Vec < _ > > () ; let mut ser = serializer . serialize_struct ("Pairs" , 2) ? ; ser . serialize_field ("pos" , & (start , end)) ? ; ser . serialize_field ("pairs" , & pairs) ? ; ser . end () } }
    };
}

impl_59!()