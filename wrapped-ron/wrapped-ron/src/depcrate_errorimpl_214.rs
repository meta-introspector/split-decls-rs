// Generated macro for impl_214 (impl)
macro_rules! Depcrate_errorimpl_214 {
() => {
// Module: crate::error
// Provides: {"impl_214"}
// Dependencies: {}
impl fmt :: Display for OneOf { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . alts { [] => write ! (f , "there are no {}" , self . none) , [a1] => write ! (f , "expected {} instead" , Identifier (a1)) , [a1 , a2] => write ! (f , "expected either {} or {} instead" , Identifier (a1) , Identifier (a2)) , [a1 , ref alts @ .. , an] => { write ! (f , "expected one of {}" , Identifier (a1)) ? ; for alt in alts { write ! (f , ", {}" , Identifier (alt)) ? ; } write ! (f , ", or {} instead" , Identifier (an)) } } } }
};
}
