macro_rules! deps {
    () => {
        SizeHint!();
    };
}

macro_rules! add_scalar {
    () => {
        deps!();
        # [doc = " Add `x` correctly to a `SizeHint`."] # [inline] pub fn add_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_add (x) ; hi = hi . and_then (| elt | elt . checked_add (x)) ; (low , hi) }
    };
}

add_scalar!()