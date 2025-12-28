macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! detach_node {
    () => {
        deps!();
        # [inline] unsafe fn detach_node < K , V > (mut node : NonNull < Node < K , V > >) { node . as_mut () . links . value . prev . as_mut () . links . value . next = node . as_ref () . links . value . next ; node . as_mut () . links . value . next . as_mut () . links . value . prev = node . as_ref () . links . value . prev ; }
    };
}

detach_node!();