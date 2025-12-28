macro_rules! deps {
    () => {
        AttributeParser!();
        AllowedTargets!();
        Stage!();
        AcceptMapping!();
        AlignParser!();
        FinalizeContext!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for AlignParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: ForeignFn) ,]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 ? ; Some (AttributeKind :: Align { align , span }) } }
    };
}

impl_167!()