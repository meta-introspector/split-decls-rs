macro_rules! deps {
    () => {
        FallibleIterator!();
        IntoFallibleIterator!();
        FlatMap!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < I , U , F > FallibleIterator for FlatMap < I , U , F > where I : FallibleIterator , U : IntoFallibleIterator < Error = I :: Error > , F : FnMut (I :: Item) -> Result < U , I :: Error > , { type Item = U :: Item ; type Error = U :: Error ; # [inline] fn next (& mut self) -> Result < Option < U :: Item > , U :: Error > { loop { if let Some (it) = & mut self . cur { if let Some (v) = it . next () ? { return Ok (Some (v)) ; } } match self . it . next () ? { Some (it) => self . cur = Some (it . into_fallible_iter ()) , None => return Ok (None) , } } } # [inline] fn try_fold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < U :: Error > , G : FnMut (B , U :: Item) -> Result < B , E > , { let mut acc = init ; if let Some (cur) = & mut self . cur { acc = cur . try_fold (acc , & mut f) ? ; self . cur = None ; } let cur = & mut self . cur ; self . it . try_fold (acc , | acc , v | { let mut it = v . into_fallible_iter () ; match it . try_fold (acc , & mut f) { Ok (acc) => Ok (acc) , Err (e) => { * cur = Some (it) ; Err (e) } } }) } }
    };
}

impl_42!()