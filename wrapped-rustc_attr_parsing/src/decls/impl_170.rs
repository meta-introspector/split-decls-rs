macro_rules! deps {
    () => {
        FinalizeContext!();
        AttributeParser!();
        AllowedTargets!();
        Stage!();
        AcceptMapping!();
        AlignStaticParser!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for AlignStaticParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Allow (Target :: ForeignStatic)]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 . 0 ? ; Some (AttributeKind :: Align { align , span }) } }
    };
}

impl_170!()