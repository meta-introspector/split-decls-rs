macro_rules! add_digit {
    () => {
        # [inline] pub (crate) fn add_digit (value : u64 , digit : u32) -> Option < u64 > { match value . checked_mul (10) { None => None , Some (n) => n . checked_add (digit as u64) , } }
    };
}

add_digit!();