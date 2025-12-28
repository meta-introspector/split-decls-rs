macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! remove_node {
    () => {
        deps!();
        # [inline] unsafe fn remove_node < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > , mut node : NonNull < Node < K , V > > ,) -> (K , V) { detach_node (node) ; push_free (free_list , node) ; node . as_mut () . take_entry () }
    };
}

remove_node!()