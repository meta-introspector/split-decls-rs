// Generated macro for impl_31 (impl)
macro_rules! Depcrate_colorimpl_31 {
() => {
// Module: crate::color
// Provides: {"impl_31"}
// Dependencies: {}
impl Display for Attribute { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut write_space = None ; for bit in 1 .. std :: mem :: size_of :: < Attribute > () * 8 { let attr = match Attribute :: from_bits (1 << bit) { Some (attr) => attr , None => continue , } ; if self . contains (attr) { if write_space . take () . is_some () { write ! (f , " ") ? ; } match attr { Attribute :: RESET => write ! (f , "reset") , Attribute :: BOLD => write ! (f , "bold") , Attribute :: NO_BOLD => write ! (f , "nobold") , Attribute :: DIM => write ! (f , "dim") , Attribute :: NO_DIM => write ! (f , "nodim") , Attribute :: UL => write ! (f , "ul") , Attribute :: NO_UL => write ! (f , "noul") , Attribute :: BLINK => write ! (f , "blink") , Attribute :: NO_BLINK => write ! (f , "noblink") , Attribute :: REVERSE => write ! (f , "reverse") , Attribute :: NO_REVERSE => write ! (f , "noreverse") , Attribute :: ITALIC => write ! (f , "italic") , Attribute :: NO_ITALIC => write ! (f , "noitalic") , Attribute :: STRIKE => write ! (f , "strike") , Attribute :: NO_STRIKE => write ! (f , "nostrike") , _ => unreachable ! ("BUG: add new attribute flag") , } ? ; write_space = Some (()) ; } } Ok (()) } }
};
}
