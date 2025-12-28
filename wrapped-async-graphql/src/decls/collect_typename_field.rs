macro_rules! deps {
    () => {
        BoxFieldFuture!();
        Object!();
        Field!();
    };
}

macro_rules! collect_typename_field {
    () => {
        deps!();
        fn collect_typename_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , object : & 'a Object , field : & 'a Positioned < Field > ,) { fields . push (async move { Ok ((field . node . response_key () . node . clone () , Value :: from (object . name . as_str ()) ,)) } . boxed () ,) }
    };
}

collect_typename_field!()