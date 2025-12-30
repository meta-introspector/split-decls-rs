// Generated macro for conservative_unescape (function)
macro_rules! Depcrate_write_literalconservative_unescape {
() => {
// Module: crate::write::literal
// Provides: {"conservative_unescape"}
// Dependencies: {}
# [doc = " Unescape a normal string into a raw string"] fn conservative_unescape (literal : & str) -> Result < String , UnescapeErr > { let mut unescaped = String :: with_capacity (literal . len ()) ; let mut chars = literal . chars () ; let mut err = false ; while let Some (ch) = chars . next () { match ch { '#' => err = true , '\\' => match chars . next () { Some ('\\') => unescaped . push ('\\') , Some ('"') => err = true , _ => return Err (UnescapeErr :: Ignore) , } , _ => unescaped . push (ch) , } } if err { Err (UnescapeErr :: Lint) } else { Ok (unescaped) } }
};
}
