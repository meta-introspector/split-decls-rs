macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        MapErr!();
        MappedErr!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < B , F , I > DoubleEndedFallibleIterator for MapErr < I , F > where I : DoubleEndedFallibleIterator , F : FnMut (I :: Error) -> B , { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , B > { self . it . next_back () . map_err (& mut self . f) } # [inline] fn try_rfold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < B > , G : FnMut (C , I :: Item) -> Result < C , E > , { self . it . try_rfold (init , | acc , v | f (acc , v) . map_err (MappedErr :: Fold)) . map_err (| e | match e { MappedErr :: It (e) => (self . f) (e) . into () , MappedErr :: Fold (e) => e , }) } }
    };
}

impl_100!();