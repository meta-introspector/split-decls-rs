macro_rules! deps {
    () => {
        PutBack!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < I > Iterator for PutBack < I > where I : Iterator , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . top { None => self . iter . next () , ref mut some => some . take () , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . iter . size_hint () , self . top . is_some () as usize) } fn count (self) -> usize { self . iter . count () + (self . top . is_some () as usize) } fn last (self) -> Option < Self :: Item > { self . iter . last () . or (self . top) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { match self . top { None => self . iter . nth (n) , ref mut some => { if n == 0 { some . take () } else { * some = None ; self . iter . nth (n - 1) } } } } fn all < G > (& mut self , mut f : G) -> bool where G : FnMut (Self :: Item) -> bool , { if let Some (elt) = self . top . take () { if ! f (elt) { return false ; } } self . iter . all (f) } fn fold < Acc , G > (mut self , init : Acc , mut f : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { let mut accum = init ; if let Some (elt) = self . top . take () { accum = f (accum , elt) ; } self . iter . fold (accum , f) } }
    };
}

impl_81!();