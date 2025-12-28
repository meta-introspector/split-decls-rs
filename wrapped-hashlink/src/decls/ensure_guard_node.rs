macro_rules! deps {
    () => {
        Node!();
        ValueLinks!();
    };
}

macro_rules! ensure_guard_node {
    () => {
        deps!();
        # [inline] unsafe fn ensure_guard_node < K , V > (head : & mut Option < NonNull < Node < K , V > > >) { if head . is_none () { let mut p = NonNull :: new_unchecked (Box :: into_raw (Box :: new (Node { entry : MaybeUninit :: uninit () , links : Links { value : ValueLinks { next : NonNull :: dangling () , prev : NonNull :: dangling () , } , } , }))) ; p . as_mut () . links . value = ValueLinks { next : p , prev : p } ; * head = Some (p) ; } }
    };
}

ensure_guard_node!()