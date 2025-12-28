macro_rules! deps {
    () => {
        SizeHint!();
    };
}

macro_rules! mul_scalar {
    () => {
        deps!();
        # [doc = " Multiply `x` correctly with a `SizeHint`."] # [inline] pub fn mul_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_mul (x) ; hi = hi . and_then (| elt | elt . checked_mul (x)) ; (low , hi) }
    };
}

mul_scalar!();