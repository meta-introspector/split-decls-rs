// Generated macro for byte (macro)
macro_rules! Depcrate_literalbyte {
() => {
// Module: crate::literal
// Provides: {"byte"}
// Dependencies: {}
macro_rules ! byte { ($ ($ p : pat) |*) => { { fn parser (i : & [u8]) -> crate :: nom :: IResult <& [u8] , u8 > { match i . split_first () { $ (Some ((& c @ $ p , rest))) |* => Ok ((rest , c)) , Some (_) => Err (nom :: Err :: Error (nom :: error :: Error :: new (i , nom :: error :: ErrorKind :: OneOf))) , None => Err (nom :: Err :: Incomplete (Needed :: new (1))) , } } parser } } }
};
}
