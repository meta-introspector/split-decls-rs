macro_rules! deps {
    () => {
        PoolIndex!();
        CombinationsWithReplacementGeneric!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < I , Idx > Iterator for CombinationsWithReplacementGeneric < I , Idx > where I : Iterator , I :: Item : Clone , Idx : PoolIndex < I :: Item > , { type Item = Idx :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . first { if ! (self . indices . borrow () . is_empty () || self . pool . get_next ()) { return None ; } self . first = false ; } else if self . increment_indices () { return None ; } Some (self . indices . extract_item (& self . pool)) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { if self . first { if ! (self . indices . borrow () . is_empty () || self . pool . get_next ()) { return None ; } self . first = false ; } else if self . increment_indices () { return None ; } for _ in 0 .. n { if self . increment_indices () { return None ; } } Some (self . indices . extract_item (& self . pool)) } fn size_hint (& self) -> (usize , Option < usize >) { let (mut low , mut upp) = self . pool . size_hint () ; low = remaining_for (low , self . first , self . indices . borrow ()) . unwrap_or (usize :: MAX) ; upp = upp . and_then (| upp | remaining_for (upp , self . first , self . indices . borrow ())) ; (low , upp) } fn count (self) -> usize { let Self { indices , pool , first , } = self ; let n = pool . count () ; remaining_for (n , first , indices . borrow ()) . unwrap () } }
    };
}

impl_194!();