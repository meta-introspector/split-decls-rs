macro_rules! delegate_iterator {
    () => {
        macro_rules ! delegate_iterator { (($ name : ident $ ($ generics : tt) *) => $ item : ty) => { impl $ ($ generics) * Iterator for $ name $ ($ generics) * { type Item = $ item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } } impl $ ($ generics) * DoubleEndedIterator for $ name $ ($ generics) * { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () } } impl $ ($ generics) * ExactSizeIterator for $ name $ ($ generics) * { # [inline] fn len (& self) -> usize { self . iter . len () } } impl $ ($ generics) * FusedIterator for $ name $ ($ generics) * { } } }
    };
}

delegate_iterator!()