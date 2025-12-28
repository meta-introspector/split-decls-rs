macro_rules! unsigned_shrinker {
    () => {
        macro_rules ! unsigned_shrinker { ($ ty : ty) => { mod shrinker { pub struct UnsignedShrinker { x : $ ty , i : $ ty , } impl UnsignedShrinker { # [allow (clippy :: new_ret_no_self)] pub fn new (x : $ ty) -> Box < dyn Iterator < Item = $ ty >> { if x == 0 { super :: empty_shrinker () } else { Box :: new (vec ! [0] . into_iter () . chain (UnsignedShrinker { x , i : x / 2 }) ,) } } } impl Iterator for UnsignedShrinker { type Item = $ ty ; fn next (& mut self) -> Option <$ ty > { if self . x - self . i < self . x { let result = Some (self . x - self . i) ; self . i /= 2 ; result } else { None } } } } } ; }
    };
}

unsigned_shrinker!()