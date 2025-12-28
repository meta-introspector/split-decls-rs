macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! drop_value_nodes {
    () => {
        deps!();
        # [inline] unsafe fn drop_value_nodes < K , V > (guard : NonNull < Node < K , V > >) { let mut cur = guard . as_ref () . links . value . prev ; while cur != guard { let prev = cur . as_ref () . links . value . prev ; cur . as_mut () . take_entry () ; let _ = Box :: from_raw (cur . as_ptr ()) ; cur = prev ; } }
    };
}

drop_value_nodes!();