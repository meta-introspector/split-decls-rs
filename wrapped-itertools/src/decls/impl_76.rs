macro_rules! deps {
    () => {
        InterleaveShortest!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < I , J > Iterator for InterleaveShortest < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let e = if self . next_coming_from_j { self . j . next () } else { self . i . next () } ; if e . is_some () { self . next_coming_from_j = ! self . next_coming_from_j ; } e } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (curr_hint , next_hint) = { let i_hint = self . i . size_hint () ; let j_hint = self . j . size_hint () ; if self . next_coming_from_j { (j_hint , i_hint) } else { (i_hint , j_hint) } } ; let (curr_lower , curr_upper) = curr_hint ; let (next_lower , next_upper) = next_hint ; let (combined_lower , combined_upper) = size_hint :: mul_scalar (size_hint :: min (curr_hint , next_hint) , 2) ; let lower = if curr_lower > next_lower { combined_lower + 1 } else { combined_lower } ; let upper = { let extra_elem = match (curr_upper , next_upper) { (_ , None) => false , (None , Some (_)) => true , (Some (curr_max) , Some (next_max)) => curr_max > next_max , } ; if extra_elem { combined_upper . and_then (| x | x . checked_add (1)) } else { combined_upper } } ; (lower , upper) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let Self { mut i , mut j , next_coming_from_j , } = self ; if next_coming_from_j { match j . next () { Some (y) => init = f (init , y) , None => return init , } } let res = i . try_fold (init , | mut acc , x | { acc = f (acc , x) ; match j . next () { Some (y) => Ok (f (acc , y)) , None => Err (acc) , } }) ; match res { Ok (val) => val , Err (val) => val , } } }
    };
}

impl_76!()