macro_rules! deps {
    () => {
        IsElement!();
        List!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T , C : IsElement < T > > Drop for List < T , C > { fn drop (& mut self) { unsafe { let guard = unprotected () ; let mut curr = self . head . load (Relaxed , guard) ; while let Some (c) = curr . as_ref () { let succ = c . next . load (Relaxed , guard) ; assert_eq ! (succ . tag () , 1) ; C :: finalize (curr . deref () , guard) ; curr = succ ; } } } }
    };
}

impl_126!()