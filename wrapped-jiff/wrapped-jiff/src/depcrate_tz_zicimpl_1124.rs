// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_tz_zicimpl_1124 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1124"}
// Dependencies: {}
impl FromStr for RuleNameP { type Err = Error ; fn from_str (name : & str) -> Result < RuleNameP , Error > { if name . is_empty () { Err (err ! ("NAME field for rule cannot be empty")) } else if name . starts_with (| ch | matches ! (ch , '0' ..='9' | '+' | '-')) { Err (err ! ("NAME field cannot begin with a digit, + or -, \
                 but {name:?} begins with one of those" ,)) } else { Ok (RuleNameP { name : name . to_string () }) } } }
};
}
