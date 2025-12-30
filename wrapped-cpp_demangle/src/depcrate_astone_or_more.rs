// Generated macro for one_or_more (function)
macro_rules! Depcrate_astone_or_more {
() => {
// Module: crate::ast
// Provides: {"one_or_more"}
// Dependencies: {}
fn one_or_more < 'a , 'b , P > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Vec < P > , IndexStr < 'b >) > where P : Parse , { let (first , mut tail) = P :: parse (ctx , subs , input) ? ; let mut results = vec ! [first] ; loop { if let Ok ((parsed , tail_tail)) = try_recurse ! (P :: parse (ctx , subs , tail)) { results . push (parsed) ; tail = tail_tail ; } else { return Ok ((results , tail)) ; } } }
};
}
