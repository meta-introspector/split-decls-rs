macro_rules! offset {
    () => {
        # [inline] fn offset < T > (base : usize) -> impl Fn ((usize , T)) -> (usize , T) { move | (i , x) | (base + i , x) }
    };
}

offset!();