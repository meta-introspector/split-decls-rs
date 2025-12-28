macro_rules! possibly_hide_cursor {
    () => {
        # [allow (unused_mut)] fn possibly_hide_cursor (out : & mut impl io :: Write , mut hide_cursor : bool) -> bool { if hide_cursor { crosstermion :: execute ! (out , crosstermion :: cursor :: Hide) . is_ok () } else { false } }
    };
}

possibly_hide_cursor!()