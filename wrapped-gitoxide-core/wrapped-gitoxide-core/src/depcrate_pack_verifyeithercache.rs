// Generated macro for EitherCache (enum)
macro_rules! Depcrate_pack_verifyEitherCache {
() => {
// Module: crate::pack::verify
// Provides: {"EitherCache"}
// Dependencies: {}
enum EitherCache < const SIZE : usize > { Left (pack :: cache :: Never) , Right (pack :: cache :: lru :: StaticLinkedList < SIZE >) , }
};
}
