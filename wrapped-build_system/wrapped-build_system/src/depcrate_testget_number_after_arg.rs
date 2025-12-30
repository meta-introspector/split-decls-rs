// Generated macro for get_number_after_arg (function)
macro_rules! Depcrate_testget_number_after_arg {
() => {
// Module: crate::test
// Provides: {"get_number_after_arg"}
// Dependencies: {}
fn get_number_after_arg (args : & mut impl Iterator < Item = String > , option : & str ,) -> Result < usize , String > { match args . next () { Some (nb) if ! nb . is_empty () => match usize :: from_str (& nb) { Ok (nb) => Ok (nb) , Err (_) => Err (format ! ("Expected a number after `{option}`, found `{nb}`")) , } , _ => Err (format ! ("Expected a number after `{option}`, found nothing")) , } }
};
}
