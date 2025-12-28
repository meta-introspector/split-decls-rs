macro_rules! iterator_methods {
    () => {
        macro_rules ! iterator_methods { ($ map_elt : expr) => { fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map ($ map_elt) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn count (self) -> usize { self . iter . len () } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth (n) . map ($ map_elt) } fn last (mut self) -> Option < Self :: Item > { self . next_back () } fn collect < C > (self) -> C where C : FromIterator < Self :: Item >, { self . iter . map ($ map_elt) . collect () } } ; }
    };
}

iterator_methods!();