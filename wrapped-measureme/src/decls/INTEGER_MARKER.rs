macro_rules! INTEGER_MARKER {
    () => {
        # [doc = " `RawEvents` that have a payload 2 value with this value are integer events."] const INTEGER_MARKER : u64 = INSTANT_MARKER - 1 ;
    };
}

INTEGER_MARKER!()