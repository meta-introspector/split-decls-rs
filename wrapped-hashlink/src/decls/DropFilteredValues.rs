macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! DropFilteredValues {
    () => {
        deps!();
        struct DropFilteredValues < 'a , K , V > { free : & 'a mut Option < NonNull < Node < K , V > > > , cur_free : Option < NonNull < Node < K , V > > > , }
    };
}

DropFilteredValues!()