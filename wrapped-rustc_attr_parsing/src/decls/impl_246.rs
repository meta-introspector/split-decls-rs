macro_rules! deps {
    () => {
        AcceptMapping!();
        AttributeParser!();
        FinalizeContext!();
        AttributeOrder!();
        AllowedTargets!();
        Single!();
        SingleAttributeParser!();
        Stage!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < T : SingleAttributeParser < S > , S : Stage > AttributeParser < S > for Single < T , S > { const ATTRIBUTES : AcceptMapping < Self , S > = & [(T :: PATH , < T as SingleAttributeParser < S > > :: TEMPLATE , | group : & mut Single < T , S > , cx , args | { if let Some (pa) = T :: convert (cx , args) { match T :: ATTRIBUTE_ORDER { AttributeOrder :: KeepInnermost => { if let Some ((_ , unused)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , cx . attr_span , unused) ; return ; } } AttributeOrder :: KeepOutermost => { if let Some ((_ , used)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , used , cx . attr_span) ; } } } group . 1 = Some ((pa , cx . attr_span)) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TYPE : AttributeType = T :: TYPE ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { Some (self . 1 ? . 0) } }
    };
}

impl_246!()