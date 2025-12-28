macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! push_free {
    () => {
        deps!();
        # [inline] unsafe fn push_free < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > , mut node : NonNull < Node < K , V > > ,) { node . as_mut () . links . free . next = * free_list ; * free_list = Some (node) ; }
    };
}

push_free!();