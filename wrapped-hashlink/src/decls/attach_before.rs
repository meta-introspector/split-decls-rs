macro_rules! deps {
    () => {
        Node!();
        ValueLinks!();
    };
}

macro_rules! attach_before {
    () => {
        deps!();
        # [inline] unsafe fn attach_before < K , V > (mut to_attach : NonNull < Node < K , V > > , mut node : NonNull < Node < K , V > >) { to_attach . as_mut () . links . value = ValueLinks { prev : node . as_ref () . links . value . prev , next : node , } ; node . as_mut () . links . value . prev = to_attach ; (* to_attach . as_mut () . links . value . prev . as_ptr ()) . links . value . next = to_attach ; }
    };
}

attach_before!();