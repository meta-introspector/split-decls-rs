macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! MapKey {
    () => {
        deps!();
        # [doc = " Only deserialize from this after peeking a '\"' byte! Otherwise it may"] # [doc = " deserialize invalid JSON successfully."] struct MapKey < 'a , R : 'a > { de : & 'a mut Deserializer < R > , }
    };
}

MapKey!();