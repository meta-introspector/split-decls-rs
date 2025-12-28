macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
        DisplayDebug!();
        Children!();
        SerDisplay!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < L : Language > Serialize for SyntaxNode < L > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_map (Some (3)) ? ; state . serialize_entry ("kind" , & SerDisplay (DisplayDebug (self . kind ()))) ? ; state . serialize_entry ("text_range" , & self . text_range ()) ? ; state . serialize_entry ("children" , & Children (self)) ? ; state . end () } }
    };
}

impl_166!();