macro_rules! deps {
    () => {
        FlattenOk!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < I , T , E > DoubleEndedIterator for FlattenOk < I , T , E > where I : DoubleEndedIterator < Item = Result < T , E > > , T : IntoIterator , T :: IntoIter : DoubleEndedIterator , { fn next_back (& mut self) -> Option < Self :: Item > { loop { if let Some (inner) = & mut self . inner_back { if let Some (item) = inner . next_back () { return Some (Ok (item)) ; } self . inner_back = None ; } match self . iter . next_back () { Some (Ok (ok)) => self . inner_back = Some (ok . into_iter ()) , Some (Err (e)) => return Some (Err (e)) , None => { if let Some (inner) = & mut self . inner_front { if let Some (item) = inner . next_back () { return Some (Ok (item)) ; } self . inner_front = None ; } else { return None ; } } } } } fn rfold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { let mut acc = match self . inner_back { Some (x) => x . rfold (init , | a , o | f (a , Ok (o))) , None => init , } ; acc = self . iter . rfold (acc , | acc , x | match x { Ok (it) => it . into_iter () . rfold (acc , | a , o | f (a , Ok (o))) , Err (e) => f (acc , Err (e)) , }) ; match self . inner_front { Some (x) => x . rfold (acc , | a , o | f (a , Ok (o))) , None => acc , } } }
    };
}

impl_231!()