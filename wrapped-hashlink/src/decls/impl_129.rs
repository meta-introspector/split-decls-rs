macro_rules! deps {
    () => {
        DropFilteredValues!();
        Node!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < K , V > DropFilteredValues < '_ , K , V > { # [inline] fn drop_later (& mut self , node : NonNull < Node < K , V > >) { unsafe { detach_node (node) ; push_free (& mut self . cur_free , node) ; } } }
    };
}

impl_129!();