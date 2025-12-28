macro_rules! deps {
    () => {
        MergeBy!();
        OrderingOrBool!();
        SizeHint!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < I , J , F > Iterator for MergeBy < I , J , F > where I : Iterator , J : Iterator , F : OrderingOrBool < I :: Item , J :: Item > , { type Item = F :: MergeResult ; fn next (& mut self) -> Option < Self :: Item > { match (self . left . next () , self . right . next ()) { (None , None) => None , (Some (left) , None) => Some (F :: left (left)) , (None , Some (right)) => Some (F :: right (right)) , (Some (left) , Some (right)) => { let (not_next , next) = self . cmp_fn . merge (left , right) ; match not_next { Some (Either :: Left (l)) => { self . left . put_back (l) ; } Some (Either :: Right (r)) => { self . right . put_back (r) ; } None => () , } Some (next) } } } fn fold < B , G > (mut self , init : B , mut f : G) -> B where Self : Sized , G : FnMut (B , Self :: Item) -> B , { let mut acc = init ; let mut left = self . left . next () ; let mut right = self . right . next () ; loop { match (left , right) { (Some (l) , Some (r)) => match self . cmp_fn . merge (l , r) { (Some (Either :: Right (r)) , x) => { acc = f (acc , x) ; left = self . left . next () ; right = Some (r) ; } (Some (Either :: Left (l)) , x) => { acc = f (acc , x) ; left = Some (l) ; right = self . right . next () ; } (None , x) => { acc = f (acc , x) ; left = self . left . next () ; right = self . right . next () ; } } , (Some (l) , None) => { self . left . put_back (l) ; acc = self . left . fold (acc , | acc , x | f (acc , F :: left (x))) ; break ; } (None , Some (r)) => { self . right . put_back (r) ; acc = self . right . fold (acc , | acc , x | f (acc , F :: right (x))) ; break ; } (None , None) => { break ; } } } acc } fn size_hint (& self) -> SizeHint { F :: size_hint (self . left . size_hint () , self . right . size_hint ()) } fn nth (& mut self , mut n : usize) -> Option < Self :: Item > { loop { if n == 0 { break self . next () ; } n -= 1 ; match (self . left . next () , self . right . next ()) { (None , None) => break None , (Some (_left) , None) => break self . left . nth (n) . map (F :: left) , (None , Some (_right)) => break self . right . nth (n) . map (F :: right) , (Some (left) , Some (right)) => { let (not_next , _) = self . cmp_fn . merge (left , right) ; match not_next { Some (Either :: Left (l)) => { self . left . put_back (l) ; } Some (Either :: Right (r)) => { self . right . put_back (r) ; } None => () , } } } } } }
    };
}

impl_361!()