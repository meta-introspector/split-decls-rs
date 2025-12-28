macro_rules! deps {
    () => {
        Hunk!();
        Side!();
    };
}

macro_rules! ancestor_hunk {
    () => {
        deps!();
        fn ancestor_hunk (start : u32 , num_lines : u32) -> Hunk { let range = start .. start + num_lines ; Hunk { before : range . clone () , after : range , side : Side :: Ancestor , } }
    };
}

ancestor_hunk!();