macro_rules! truncate_non_escaped_trailing_spaces {
    () => {
        # [doc = " We always copy just because that's ultimately needed anyway, not because we always have to."] fn truncate_non_escaped_trailing_spaces (buf : & [u8]) -> & [u8] { let mut last_space_pos = None ; let mut bytes = buf . iter () . enumerate () ; while let Some ((pos , b)) = bytes . next () { match * b { b' ' => { last_space_pos . get_or_insert (pos) ; continue ; } b'\\' => { if bytes . next () . is_none () { return buf ; } } _ => { } } last_space_pos = None ; } if let Some (pos) = last_space_pos { & buf [.. pos] } else { buf } }
    };
}

truncate_non_escaped_trailing_spaces!()