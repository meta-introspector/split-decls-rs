// Generated macro for write_punycode_label (function)
macro_rules! Depcrate_uts46write_punycode_label {
() => {
// Module: crate::uts46
// Provides: {"write_punycode_label"}
// Dependencies: {}
fn write_punycode_label < W : Write + ? Sized > (label : & [char] , sink : & mut W ,) -> Result < () , ProcessingError > { sink . write_str ("xn--") ? ; crate :: punycode :: encode_into :: < _ , _ , InternalCaller > (label . iter () . copied () , sink) ? ; Ok (()) }
};
}
