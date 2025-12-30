// Generated macro for repeat_range_literals (function)
macro_rules! Depcrate_literalsrepeat_range_literals {
() => {
// Module: crate::literals
// Provides: {"repeat_range_literals"}
// Dependencies: {}
fn repeat_range_literals < F : FnMut (& Expr , & mut Literals) > (e : & Expr , min : u32 , max : Option < u32 > , greedy : bool , lits : & mut Literals , mut f : F ,) { use Expr :: * ; if min == 0 { f (& Repeat { e : Box :: new (e . clone ()) , r : Repeater :: ZeroOrMore , greedy : greedy , } , lits) ; } else { if min > 0 { let n = cmp :: min (lits . limit_size , min as usize) ; let es = iter :: repeat (e . clone ()) . take (n) . collect () ; f (& Concat (es) , lits) ; if n < min as usize { lits . cut () ; } } if max . map_or (true , | max | min < max) { lits . cut () ; } } }
};
}
