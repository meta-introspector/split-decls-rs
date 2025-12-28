macro_rules! deps {
    () => {
        InDomainFn!();
        TransformFn!();
    };
}

macro_rules! TransformCallback {
    () => {
        deps!();
        pub struct TransformCallback < 'a > { pub name : CString , pub transform_fn : TransformFn < 'a > , pub in_domain_fn : Option < InDomainFn > , }
    };
}

TransformCallback!();