macro_rules! double_ended_iterator_methods {
    () => {
        macro_rules ! double_ended_iterator_methods { ($ map_elt : expr) => { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map ($ map_elt) } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth_back (n) . map ($ map_elt) } } ; }
    };
}

double_ended_iterator_methods!();