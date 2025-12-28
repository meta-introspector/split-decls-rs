macro_rules! deps {
    () => {
        ChainState!();
        DoubleEndedFallibleIterator!();
        Chain!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T , U > DoubleEndedFallibleIterator for Chain < T , U > where T : DoubleEndedFallibleIterator , U : DoubleEndedFallibleIterator < Item = T :: Item , Error = T :: Error > , { # [inline] fn next_back (& mut self) -> Result < Option < T :: Item > , T :: Error > { match self . state { ChainState :: Both => match self . back . next_back () ? { Some (e) => Ok (Some (e)) , None => { self . state = ChainState :: Front ; self . front . next_back () } } , ChainState :: Front => self . front . next_back () , ChainState :: Back => self . back . next_back () , } } # [inline] fn try_rfold < B , E , F > (& mut self , init : B , mut f : F) -> Result < B , E > where E : From < T :: Error > , F : FnMut (B , T :: Item) -> Result < B , E > , { match self . state { ChainState :: Both => { let init = self . back . try_rfold (init , & mut f) ? ; self . state = ChainState :: Front ; self . front . try_rfold (init , f) } ChainState :: Front => self . front . try_rfold (init , f) , ChainState :: Back => self . back . try_rfold (init , f) , } } }
    };
}

impl_21!()