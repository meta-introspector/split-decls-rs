macro_rules! deps {
    () => {
        Interleave!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < I , J > Iterator for Interleave < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . next_coming_from_j = ! self . next_coming_from_j ; if self . next_coming_from_j { match self . i . next () { None => self . j . next () , r => r , } } else { match self . j . next () { None => self . i . next () , r => r , } } } fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add (self . i . size_hint () , self . j . size_hint ()) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let Self { mut i , mut j , next_coming_from_j , } = self ; if next_coming_from_j { match j . next () { Some (y) => init = f (init , y) , None => return i . fold (init , f) , } } let res = i . try_fold (init , | mut acc , x | { acc = f (acc , x) ; match j . next () { Some (y) => Ok (f (acc , y)) , None => Err (acc) , } }) ; match res { Ok (acc) => j . fold (acc , f) , Err (acc) => i . fold (acc , f) , } } }
    };
}

impl_72!();