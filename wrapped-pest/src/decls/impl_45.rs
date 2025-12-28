macro_rules! deps {
    () => {
        Pair!();
        RuleType!();
        Error!();
        Pairs!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (feature = "pretty-print")] impl < R : RuleType > :: serde :: Serialize for Pair < '_ , R > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { let start = self . pos (self . start) ; let end = self . pos (self . pair ()) ; let rule = format ! ("{:?}" , self . as_rule ()) ; let inner = self . clone () . into_inner () ; let mut ser = serializer . serialize_struct ("Pairs" , 3) ? ; ser . serialize_field ("pos" , & (start , end)) ? ; ser . serialize_field ("rule" , & rule) ? ; if inner . peek () . is_none () { ser . serialize_field ("inner" , & self . as_str ()) ? ; } else { ser . serialize_field ("inner" , & inner) ? ; } ser . end () } }
    };
}

impl_45!()