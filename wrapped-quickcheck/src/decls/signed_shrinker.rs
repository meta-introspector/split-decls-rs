macro_rules! signed_shrinker {
    () => {
        macro_rules ! signed_shrinker { ($ ty : ty) => { mod shrinker { pub struct SignedShrinker { x : $ ty , i : $ ty , } impl SignedShrinker { # [allow (clippy :: new_ret_no_self)] pub fn new (x : $ ty) -> Box < dyn Iterator < Item = $ ty >> { if x == 0 { super :: empty_shrinker () } else { let shrinker = SignedShrinker { x , i : x / 2 } ; let mut items = vec ! [0] ; if shrinker . i < 0 && shrinker . x != <$ ty >:: MIN { items . push (shrinker . x . abs ()) ; } Box :: new (items . into_iter () . chain (shrinker)) } } } impl Iterator for SignedShrinker { type Item = $ ty ; fn next (& mut self) -> Option <$ ty > { if self . x == <$ ty >:: MIN || (self . x - self . i) . abs () < self . x . abs () { let result = Some (self . x - self . i) ; self . i /= 2 ; result } else { None } } } } } ; }
    };
}

signed_shrinker!()