macro_rules! deps {
    () => {
        FallibleIterator!();
        IntoFallibleIterator!();
        Flatten!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < I > FallibleIterator for Flatten < I > where I : FallibleIterator , I :: Item : IntoFallibleIterator < Error = I :: Error > , { type Item = < I :: Item as IntoFallibleIterator > :: Item ; type Error = < I :: Item as IntoFallibleIterator > :: Error ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { loop { if let Some (it) = & mut self . cur { if let Some (v) = it . next () ? { return Ok (Some (v)) ; } } match self . it . next () ? { Some (it) => self . cur = Some (it . into_fallible_iter ()) , None => return Ok (None) , } } } # [inline] fn try_fold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < Self :: Error > , G : FnMut (B , Self :: Item) -> Result < B , E > , { let mut acc = init ; if let Some (cur) = & mut self . cur { acc = cur . try_fold (acc , & mut f) ? ; self . cur = None ; } let cur = & mut self . cur ; self . it . try_fold (acc , | acc , v | { let mut it = v . into_fallible_iter () ; match it . try_fold (acc , & mut f) { Ok (acc) => Ok (acc) , Err (e) => { * cur = Some (it) ; Err (e) } } }) } }
    };
}

impl_86!();