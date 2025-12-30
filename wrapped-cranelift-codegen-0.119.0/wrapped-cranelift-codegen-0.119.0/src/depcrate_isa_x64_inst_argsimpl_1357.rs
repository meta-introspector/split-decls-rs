// Generated macro for impl_1357 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1357 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1357"}
// Dependencies: {}
impl fmt :: Debug for CC { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let name = match self { CC :: O => "o" , CC :: NO => "no" , CC :: B => "b" , CC :: NB => "nb" , CC :: Z => "z" , CC :: NZ => "nz" , CC :: BE => "be" , CC :: NBE => "nbe" , CC :: S => "s" , CC :: NS => "ns" , CC :: L => "l" , CC :: NL => "nl" , CC :: LE => "le" , CC :: NLE => "nle" , CC :: P => "p" , CC :: NP => "np" , } ; write ! (fmt , "{name}") } }
};
}
