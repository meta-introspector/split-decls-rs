macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! get_cursor {
    () => {
        deps!();
        # [cfg (not (span_locations))] fn get_cursor (src : & str) -> Cursor { Cursor { rest : src } }
    };
}

get_cursor!();