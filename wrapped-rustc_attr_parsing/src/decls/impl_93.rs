macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        OnDuplicate!();
        Stage!();
        AllowedTargets!();
        ExportStableParser!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ExportStableParser { const PATH : & [Symbol] = & [sym :: export_stable] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: ExportStable ; }
    };
}

impl_93!()