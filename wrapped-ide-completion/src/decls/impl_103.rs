macro_rules! deps {
    () => {
        QualifierCtx!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl QualifierCtx { pub (crate) fn none (& self) -> bool { self . async_tok . is_none () && self . unsafe_tok . is_none () && self . safe_tok . is_none () && self . vis_node . is_none () } }
    };
}

impl_103!();