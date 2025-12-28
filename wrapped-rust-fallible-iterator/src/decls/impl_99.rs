macro_rules! deps {
    () => {
        FallibleIterator!();
        MappedErr!();
        MapErr!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < B , F , I > FallibleIterator for MapErr < I , F > where I : FallibleIterator , F : FnMut (I :: Error) -> B , { type Item = I :: Item ; type Error = B ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , B > { self . it . next () . map_err (& mut self . f) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn count (mut self) -> Result < usize , B > { self . it . count () . map_err (& mut self . f) } # [inline] fn last (mut self) -> Result < Option < I :: Item > , B > { self . it . last () . map_err (& mut self . f) } # [inline] fn nth (& mut self , n : usize) -> Result < Option < I :: Item > , B > { self . it . nth (n) . map_err (& mut self . f) } # [inline] fn try_fold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < B > , G : FnMut (C , I :: Item) -> Result < C , E > , { self . it . try_fold (init , | acc , v | f (acc , v) . map_err (MappedErr :: Fold)) . map_err (| e | match e { MappedErr :: It (e) => (self . f) (e) . into () , MappedErr :: Fold (e) => e , }) } }
    };
}

impl_99!()