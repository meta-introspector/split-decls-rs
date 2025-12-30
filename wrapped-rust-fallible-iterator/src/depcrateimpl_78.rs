// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < T , E , I > DoubleEndedFallibleIterator for Convert < I > where I : DoubleEndedIterator < Item = Result < T , E > > , { # [inline] fn next_back (& mut self) -> Result < Option < T > , E > { match self . 0 . next_back () { Some (Ok (i)) => Ok (Some (i)) , Some (Err (e)) => Err (e) , None => Ok (None) , } } # [inline] fn try_rfold < B , E2 , F > (& mut self , init : B , mut f : F) -> Result < B , E2 > where E2 : From < E > , F : FnMut (B , T) -> Result < B , E2 > , { self . 0 . try_rfold (init , | acc , v | f (acc , v ?)) } }
};
}
