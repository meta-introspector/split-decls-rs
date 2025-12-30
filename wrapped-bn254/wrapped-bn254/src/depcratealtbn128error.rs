// Generated macro for AltBn128Error (enum)
macro_rules! DepcrateAltBn128Error {
() => {
// Module: crate
// Provides: {"AltBn128Error"}
// Dependencies: {}
# [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum AltBn128Error { # [error ("The input data is invalid")] InvalidInputData , # [error ("Invalid group data")] GroupError , # [error ("Slice data is going out of input data bounds")] SliceOutOfBounds , # [error ("Unexpected error")] UnexpectedError , # [error ("Failed to convert a byte slice into a vector {0:?}")] TryIntoVecError (Vec < u8 >) , # [error ("Failed to convert projective to affine g1")] ProjectiveToG1Failed , }
};
}
