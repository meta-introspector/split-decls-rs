macro_rules! deps {
    () => {
        IntersperseElement!();
        IntersperseWith!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < I , ElemF > Iterator for IntersperseWith < I , ElemF > where I : Iterator , ElemF : IntersperseElement < I :: Item > , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let Self { element , iter , peek , } = self ; match peek { Some (item @ Some (_)) => item . take () , Some (None) => match iter . next () { new @ Some (_) => { * peek = Some (new) ; Some (element . generate ()) } None => None , } , None => { * peek = Some (None) ; iter . next () } } } fn size_hint (& self) -> (usize , Option < usize >) { let mut sh = self . iter . size_hint () ; sh = size_hint :: add (sh , sh) ; match self . peek { Some (Some (_)) => size_hint :: add_scalar (sh , 1) , Some (None) => sh , None => size_hint :: sub_scalar (sh , 1) , } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let Self { mut element , mut iter , peek , } = self ; let mut accum = init ; if let Some (x) = peek . unwrap_or_else (| | iter . next ()) { accum = f (accum , x) ; } iter . fold (accum , | accum , x | { let accum = f (accum , element . generate ()) ; f (accum , x) }) } }
    };
}

impl_304!();