macro_rules! deps {
    () => {
        FallibleIterator!();
        Peekable!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < I > FallibleIterator for Peekable < I > where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if let Some (next) = self . next . take () { return Ok (Some (next)) ; } self . it . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let mut hint = self . it . size_hint () ; if self . next . is_some () { hint . 0 = hint . 0 . saturating_add (1) ; hint . 1 = hint . 1 . and_then (| h | h . checked_add (1)) ; } hint } # [inline] fn try_fold < B , E , F > (& mut self , init : B , mut f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , I :: Item) -> Result < B , E > , { let mut acc = init ; if let Some (v) = self . next . take () { acc = f (acc , v) ? ; } self . it . try_fold (acc , f) } }
    };
}

impl_105!();