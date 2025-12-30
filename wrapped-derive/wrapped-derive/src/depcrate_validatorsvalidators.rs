// Generated macro for Validators (struct)
macro_rules! Depcrate_validatorsValidators {
() => {
// Module: crate::validators
// Provides: {"Validators"}
// Dependencies: {}
# [derive (FromMeta , Default , Clone)] pub struct Validators { # [darling (default)] multiple_of : Option < Number > , # [darling (default)] min_password_strength : Option < u8 > , # [darling (default)] maximum : Option < Number > , # [darling (default)] minimum : Option < Number > , # [darling (default)] max_length : Option < usize > , # [darling (default)] min_length : Option < usize > , # [darling (default)] max_items : Option < usize > , # [darling (default)] min_items : Option < usize > , # [darling (default)] chars_max_length : Option < usize > , # [darling (default)] chars_min_length : Option < usize > , # [darling (default)] email : bool , # [darling (default)] url : bool , # [darling (default)] ip : bool , # [darling (default)] regex : Option < String > , # [darling (default)] uuid : Option < UuidVersionValidation > , # [darling (default , multiple)] custom : Vec < Expr > , # [darling (default)] list : bool , }
};
}
