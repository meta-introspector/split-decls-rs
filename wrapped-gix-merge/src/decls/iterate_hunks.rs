macro_rules! deps {
    () => {
        Hunk!();
        Side!();
    };
}

macro_rules! iterate_hunks {
    () => {
        deps!();
        # [doc = " Return an iterator over `(token_idx, hunk_idx, hunk_side)` from `hunks`."] fn iterate_hunks (hunks : & [Hunk]) -> impl Iterator < Item = (u32 , usize , Side) > + '_ { hunks . iter () . enumerate () . flat_map (| (hunk_idx , hunk) | { match hunk . side { Side :: Current | Side :: Other => & hunk . after , Side :: Ancestor => & hunk . before , } . clone () . map (move | idx | (idx , hunk_idx , hunk . side)) }) }
    };
}

iterate_hunks!()