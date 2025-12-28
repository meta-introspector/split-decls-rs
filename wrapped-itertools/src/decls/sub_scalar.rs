macro_rules! deps {
    () => {
        SizeHint!();
    };
}

macro_rules! sub_scalar {
    () => {
        deps!();
        # [doc = " Subtract `x` correctly from a `SizeHint`."] # [inline] pub fn sub_scalar (sh : SizeHint , x : usize) -> SizeHint { let (mut low , mut hi) = sh ; low = low . saturating_sub (x) ; hi = hi . map (| elt | elt . saturating_sub (x)) ; (low , hi) }
    };
}

sub_scalar!();