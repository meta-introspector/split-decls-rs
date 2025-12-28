macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Masks { # [inline] fn new < T : IndexRange > (range : T , length : usize) -> Masks { let start = range . start () . unwrap_or (0) ; let end = range . end () . unwrap_or (length) ; assert ! (start <= end && end <= length , "invalid range {}..{} for a fixedbitset of size {}" , start , end , length) ; let (first_block , first_rem) = div_rem (start , BITS) ; let (last_block , last_rem) = div_rem (end , BITS) ; Masks { first_block , first_mask : usize :: MAX << first_rem , last_block , last_mask : (usize :: MAX >> 1) >> (BITS - last_rem - 1) , } } }
    };
}

impl_39!()