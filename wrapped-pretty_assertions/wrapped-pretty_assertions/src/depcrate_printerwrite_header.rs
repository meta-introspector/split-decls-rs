// Generated macro for write_header (function)
macro_rules! Depcrate_printerwrite_header {
() => {
// Module: crate::printer
// Provides: {"write_header"}
// Dependencies: {}
# [doc = " Present the diff output for two mutliline strings in a pretty, colorised manner."] pub (crate) fn write_header (f : & mut fmt :: Formatter) -> fmt :: Result { writeln ! (f , "{} {} {} / {} {} :" , "Diff" . bold () , SIGN_LEFT . red () . linger () , "left" . resetting () , "right" . green () . linger () , SIGN_RIGHT . resetting () ,) }
};
}
