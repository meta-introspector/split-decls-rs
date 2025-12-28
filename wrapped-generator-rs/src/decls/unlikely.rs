macro_rules! unlikely {
    () => {
        # [inline] pub (crate) fn unlikely (b : bool) -> bool { if b { cold () } b }
    };
}

unlikely!()