// Generated macro for case (macro)
macro_rules! Depcratecase {
() => {
// Module: crate
// Provides: {"case"}
// Dependencies: {}
# [doc = " The variant of `case` from a token."] # [doc = ""] # [doc = " The token associated with each variant is the variant written in snake case."] # [cfg (feature = "random")] # [macro_export] macro_rules ! case { (snake) => { convert_case :: Case :: Snake } ; (constant) => { convert_case :: Case :: Constant } ; (upper_snake) => { convert_case :: Case :: UpperSnake } ; (ada) => { convert_case :: Case :: Ada ; } ; (kebab) => { convert_case :: Case :: Kebab } ; (cobol) => { convert_case :: Case :: Cobol } ; (upper_kebab) => { convert_case :: Case :: UpperKebab } ; (train) => { convert_case :: Case :: Train } ; (flat) => { convert_case :: Case :: Flat } ; (upper_flat) => { convert_case :: Case :: UpperFlat } ; (pascal) => { convert_case :: Case :: Pascal } ; (upper_camel) => { convert_case :: Case :: UpperCamel } ; (camel) => { convert_case :: Case :: Camel } ; (lower) => { convert_case :: Case :: Lower } ; (upper) => { convert_case :: Case :: Upper } ; (title) => { convert_case :: Case :: Title } ; (sentence) => { convert_case :: Case :: Sentence } ; (alternating) => { convert_case :: Case :: Alternating } ; (toggle) => { convert_case :: Case :: Toggle } ; (random) => { convert_case :: Case :: Random } ; (psuedo_random) => { convert_case :: Case :: PsuedoRandom } ; }
};
}
