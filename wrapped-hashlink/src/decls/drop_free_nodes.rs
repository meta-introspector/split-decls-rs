macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! drop_free_nodes {
    () => {
        deps!();
        # [inline] unsafe fn drop_free_nodes < K , V > (mut free : Option < NonNull < Node < K , V > > >) { while let Some (some_free) = free { let next_free = some_free . as_ref () . links . free . next ; let _ = Box :: from_raw (some_free . as_ptr ()) ; free = next_free ; } }
    };
}

drop_free_nodes!();