// Generated macro for Piece (enum)
macro_rules! DepcratePiece {
() => {
// Module: crate
// Provides: {"Piece"}
// Dependencies: {}
# [doc = " A piece is a portion of the format string which represents the next part"] # [doc = " to emit. These are emitted as a stream by the `Parser` class."] # [derive (Clone , Debug , PartialEq)] pub enum Piece < 'a > { # [doc = " A literal string which should directly be emitted"] String (& 'a str) , # [doc = " This describes that formatting should process the next argument (as"] # [doc = " specified inside) for emission."] NextArgument (Box < Argument < 'a > >) , }
};
}
