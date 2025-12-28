macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        WithoutArgs!();
        AcceptContext!();
        Stage!();
        SingleAttributeParser!();
        ArgParser!();
        AttributeOrder!();
        OnDuplicate!();
        AllowedTargets!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl < T : NoArgsAttributeParser < S > , S : Stage > SingleAttributeParser < S > for WithoutArgs < T , S > { const PATH : & [Symbol] = T :: PATH ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = T :: ON_DUPLICATE ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TEMPLATE : AttributeTemplate = template ! (Word) ; const TYPE : AttributeType = T :: TYPE ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; } Some (T :: CREATE (cx . attr_span)) } }
    };
}

impl_253!()