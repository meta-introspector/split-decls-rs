macro_rules! deps {
    () => {
        OnDuplicate!();
        Stage!();
        NoMangleParser!();
        NoArgsAttributeParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for NoMangleParser { const PATH : & [Symbol] = & [sym :: no_mangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Allow (Target :: Static) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoMangle ; }
    };
}

impl_43!();