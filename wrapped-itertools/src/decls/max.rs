macro_rules! deps {
    () => {
        SizeHint!();
    };
}

macro_rules! max {
    () => {
        deps!();
        # [doc = " Return the maximum"] # [inline] pub fn max (a : SizeHint , b : SizeHint) -> SizeHint { let (a_lower , a_upper) = a ; let (b_lower , b_upper) = b ; let lower = cmp :: max (a_lower , b_lower) ; let upper = match (a_upper , b_upper) { (Some (x) , Some (y)) => Some (cmp :: max (x , y)) , _ => None , } ; (lower , upper) }
    };
}

max!()