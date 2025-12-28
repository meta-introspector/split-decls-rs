macro_rules! deps {
    () => {
        Stage!();
        FinalizeContext!();
        AttributeParser!();
        AcceptMapping!();
        AllowedTargets!();
        Combine!();
        CombineAttributeParser!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < T : CombineAttributeParser < S > , S : Stage > AttributeParser < S > for Combine < T , S > { const ATTRIBUTES : AcceptMapping < Self , S > = & [(T :: PATH , T :: TEMPLATE , | group : & mut Combine < T , S > , cx , args | { group . first_span . get_or_insert (cx . attr_span) ; group . items . extend (T :: extend (cx , args)) })] ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TYPE : AttributeType = T :: TYPE ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if let Some (first_span) = self . first_span { Some (T :: CONVERT (self . items , first_span)) } else { None } } }
    };
}

impl_258!();