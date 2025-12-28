macro_rules! deps {
    () => {
        OnDuplicate!();
        AllowedTargets!();
        AsPtrParser!();
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for AsPtrParser { const PATH : & [Symbol] = & [sym :: rustc_as_ptr] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AsPtr ; }
    };
}

impl_106!();