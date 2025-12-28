macro_rules! deps {
    () => {
        OnDuplicate!();
        Stage!();
        UnsafeSpecializationMarkerParser!();
        NoArgsAttributeParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for UnsafeSpecializationMarkerParser { const PATH : & [Symbol] = & [sym :: rustc_unsafe_specialization_marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: UnsafeSpecializationMarker ; }
    };
}

impl_220!()