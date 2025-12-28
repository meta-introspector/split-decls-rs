macro_rules! deps {
    () => {
        HomogeneousTuple!();
        TupleWindows!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl < I , T > Iterator for TupleWindows < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple + Clone , T :: Item : Clone , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if T :: num_items () == 1 { return T :: collect_from_iter_no_buf (& mut self . iter) ; } if let Some (new) = self . iter . next () { if let Some (ref mut last) = self . last { last . left_shift_push (new) ; Some (last . clone ()) } else { use std :: iter :: once ; let iter = once (new) . chain (& mut self . iter) ; self . last = T :: collect_from_iter_no_buf (iter) ; self . last . clone () } } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let mut sh = self . iter . size_hint () ; if self . last . is_none () { sh = size_hint :: sub_scalar (sh , T :: num_items () - 1) ; } sh } }
    };
}

impl_516!()