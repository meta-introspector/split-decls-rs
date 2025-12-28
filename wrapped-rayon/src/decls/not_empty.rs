macro_rules! not_empty {
    () => {
        # [inline] fn not_empty (s : & & str) -> bool { ! s . is_empty () }
    };
}

not_empty!()