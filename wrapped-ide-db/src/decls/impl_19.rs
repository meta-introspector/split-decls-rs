macro_rules! deps {
    () => {
        AssistResolveStrategy!();
        AssistId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl AssistResolveStrategy { pub fn should_resolve (& self , id : & AssistId) -> bool { match self { AssistResolveStrategy :: None => false , AssistResolveStrategy :: All => true , AssistResolveStrategy :: Single (single_resolve) => { single_resolve . assist_id == id . 0 && single_resolve . assist_kind == id . 1 && single_resolve . assist_subtype == id . 2 } } } }
    };
}

impl_19!()