macro_rules! deps {
    () => {
        Language!();
        SyntaxToken!();
        DisplayDebug!();
        SerDisplay!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < L : Language > Serialize for SyntaxToken < L > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_map (Some (3)) ? ; state . serialize_entry ("kind" , & SerDisplay (DisplayDebug (self . kind ()))) ? ; state . serialize_entry ("text_range" , & self . text_range ()) ? ; state . serialize_entry ("text" , & self . text ()) ? ; state . end () } }
    };
}

impl_167!()