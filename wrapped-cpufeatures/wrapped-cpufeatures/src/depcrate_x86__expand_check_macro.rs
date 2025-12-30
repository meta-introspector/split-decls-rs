// Generated macro for __expand_check_macro (macro)
macro_rules! Depcrate_x86__expand_check_macro {
() => {
// Module: crate::x86
// Provides: {"__expand_check_macro"}
// Dependencies: {}
macro_rules ! __expand_check_macro { ($ (($ name : tt , $ reg_cap : tt $ (, $ i : expr , $ reg : ident , $ offset : expr) *)) ,* $ (,) ?) => { # [macro_export] # [doc (hidden)] macro_rules ! check { $ (($ cr : expr , $ name) => { { let reg_cap = match $ reg_cap { "xmm" => $ crate :: __xgetbv ! ($ cr , 0b10) , "ymm" => $ crate :: __xgetbv ! ($ cr , 0b110) , "zmm" => $ crate :: __xgetbv ! ($ cr , 0b1110_0110) , _ => true , } ; reg_cap $ (& ($ cr [$ i] .$ reg & (1 << $ offset) != 0)) * } } ;) * } } ; }
};
}
