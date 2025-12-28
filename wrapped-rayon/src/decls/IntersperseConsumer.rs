macro_rules! IntersperseConsumer {
    () => {
        struct IntersperseConsumer < C , T > { base : C , item : T , clone_first : Cell < bool > , }
    };
}

IntersperseConsumer!()