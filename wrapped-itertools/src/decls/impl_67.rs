macro_rules! deps {
    () => {
        MultiProduct!();
        MultiProductInner!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < I > Iterator for MultiProduct < I > where I : Iterator + Clone , I :: Item : Clone , { type Item = Vec < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { let inner = self . 0 . as_mut () ? ; match & mut inner . cur { Populated (values) => { debug_assert ! (! inner . iters . is_empty ()) ; for (iter , item) in inner . iters . iter_mut () . zip (values . iter_mut ()) . rev () { if let Some (new) = iter . iter . next () { * item = new ; return Some (values . clone ()) ; } else { iter . iter = iter . iter_orig . clone () ; * item = iter . iter . next () . unwrap () ; } } self . 0 = ProductEnded ; None } NotYetPopulated => { let next : Option < Vec < _ > > = inner . iters . iter_mut () . map (| i | i . iter . next ()) . collect () ; if next . is_none () || inner . iters . is_empty () { self . 0 = ProductEnded ; } else { inner . cur . clone_from (& next) ; } next } } } fn count (self) -> usize { match self . 0 { ProductEnded => 0 , ProductInProgress (MultiProductInner { iters , cur : NotYetPopulated , }) => iters . into_iter () . map (| iter | iter . iter_orig . count ()) . try_fold (1 , | product , count | { if count == 0 { None } else { Some (product * count) } }) . unwrap_or_default () , ProductInProgress (MultiProductInner { iters , cur : Populated (_) , }) => iters . into_iter () . fold (0 , | mut acc , iter | { if acc != 0 { acc *= iter . iter_orig . count () ; } acc + iter . iter . count () }) , } } fn size_hint (& self) -> (usize , Option < usize >) { match & self . 0 { ProductEnded => (0 , Some (0)) , ProductInProgress (MultiProductInner { iters , cur : NotYetPopulated , }) => iters . iter () . map (| iter | iter . iter_orig . size_hint ()) . fold ((1 , Some (1)) , size_hint :: mul) , ProductInProgress (MultiProductInner { iters , cur : Populated (_) , }) => { if let [first , tail @ ..] = & iters [..] { tail . iter () . fold (first . iter . size_hint () , | mut sh , iter | { sh = size_hint :: mul (sh , iter . iter_orig . size_hint ()) ; size_hint :: add (sh , iter . iter . size_hint ()) }) } else { unreachable ! () } } } } fn last (self) -> Option < Self :: Item > { let MultiProductInner { iters , cur } = self . 0 ? ; if let Populated (values) = cur { let mut count = iters . len () ; let last = iters . into_iter () . zip (values) . map (| (i , value) | { i . iter . last () . unwrap_or_else (| | { count -= 1 ; value }) }) . collect () ; if count == 0 { None } else { Some (last) } } else { iters . into_iter () . map (| i | i . iter . last ()) . collect () } } }
    };
}

impl_67!();