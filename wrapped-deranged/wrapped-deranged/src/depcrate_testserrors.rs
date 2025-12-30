// Generated macro for errors (function)
macro_rules! Depcrate_testserrors {
() => {
// Module: crate::tests
// Provides: {"errors"}
// Dependencies: {}
# [test] fn errors () { assert_eq ! (TryFromIntError . to_string () , "out of range integral type conversion attempted") ; assert_eq ! (TryFromIntError . clone () , TryFromIntError) ; assert_eq ! (format ! ("{TryFromIntError:?}") , "TryFromIntError") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: Empty , } . to_string () , "cannot parse integer from empty string") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: InvalidDigit , } . to_string () , "invalid digit found in string") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: PosOverflow , } . to_string () , "number too large to fit in target type") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: NegOverflow , } . to_string () , "number too small to fit in target type") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: Zero , } . to_string () , "number would be zero for non-zero type") ; assert_eq ! (format ! ("{:?}" , ParseIntError { kind : IntErrorKind :: Empty }) , "ParseIntError { kind: Empty }") ; assert_eq ! (ParseIntError { kind : IntErrorKind :: Empty } . clone () , ParseIntError { kind : IntErrorKind :: Empty }) ; assert_eq ! (ParseIntError { kind : IntErrorKind :: Empty } . kind () , & IntErrorKind :: Empty) ; }
};
}
