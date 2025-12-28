macro_rules! deps {
    () => {
        ZipLongest!();
        EitherOrBoth!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl < T , U > Iterator for ZipLongest < T , U > where T : Iterator , U : Iterator , { type Item = EitherOrBoth < T :: Item , U :: Item > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match (self . a . next () , self . b . next ()) { (None , None) => None , (Some (a) , None) => Some (EitherOrBoth :: Left (a)) , (None , Some (b)) => Some (EitherOrBoth :: Right (b)) , (Some (a) , Some (b)) => Some (EitherOrBoth :: Both (a , b)) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: max (self . a . size_hint () , self . b . size_hint ()) } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let Self { mut a , mut b } = self ; let res = a . try_fold (init , | init , a | match b . next () { Some (b) => Ok (f (init , EitherOrBoth :: Both (a , b))) , None => Err (f (init , EitherOrBoth :: Left (a))) , }) ; match res { Ok (acc) => b . map (EitherOrBoth :: Right) . fold (acc , f) , Err (acc) => a . map (EitherOrBoth :: Left) . fold (acc , f) , } } }
    };
}

impl_577!()