macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        ConstStabilityIndirectParser!();
        OnDuplicate!();
        AllowedTargets!();
        Stage!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ConstStabilityIndirectParser { const PATH : & [Symbol] = & [sym :: rustc_const_stable_indirect] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) ,]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: ConstStabilityIndirect ; }
    };
}

impl_190!()