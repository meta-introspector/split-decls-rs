macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! pop_free {
    () => {
        deps!();
        # [inline] unsafe fn pop_free < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > ,) -> Option < NonNull < Node < K , V > > > { if let Some (free) = * free_list { * free_list = free . as_ref () . links . free . next ; Some (free) } else { None } }
    };
}

pop_free!();