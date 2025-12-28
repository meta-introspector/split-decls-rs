macro_rules! deps {
    () => {
        FlattenOk!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < I , T , E > Iterator for FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > , T : IntoIterator , { type Item = Result < T :: Item , E > ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some (inner) = & mut self . inner_front { if let Some (item) = inner . next () { return Some (Ok (item)) ; } self . inner_front = None ; } match self . iter . next () { Some (Ok (ok)) => self . inner_front = Some (ok . into_iter ()) , Some (Err (e)) => return Some (Err (e)) , None => { if let Some (inner) = & mut self . inner_back { if let Some (item) = inner . next () { return Some (Ok (item)) ; } self . inner_back = None ; } else { return None ; } } } } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let mut acc = match self . inner_front { Some (x) => x . fold (init , | a , o | f (a , Ok (o))) , None => init , } ; acc = self . iter . fold (acc , | acc , x | match x { Ok (it) => it . into_iter () . fold (acc , | a , o | f (a , Ok (o))) , Err (e) => f (acc , Err (e)) , }) ; match self . inner_back { Some (x) => x . fold (acc , | a , o | f (a , Ok (o))) , None => acc , } } fn size_hint (& self) -> (usize , Option < usize >) { let inner_hint = | inner : & Option < T :: IntoIter > | { inner . as_ref () . map (Iterator :: size_hint) . unwrap_or ((0 , Some (0))) } ; let inner_front = inner_hint (& self . inner_front) ; let inner_back = inner_hint (& self . inner_back) ; let outer = match self . iter . size_hint () { (0 , Some (0)) => (0 , Some (0)) , _ => (0 , None) , } ; size_hint :: add (size_hint :: add (inner_front , inner_back) , outer) } }
    };
}

impl_230!()