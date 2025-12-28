macro_rules! Marker {
    () => {
        # [doc = " See [`Parser::start`]."] pub (crate) struct Marker { pos : u32 , bomb : DropBomb , }
    };
}

Marker!()