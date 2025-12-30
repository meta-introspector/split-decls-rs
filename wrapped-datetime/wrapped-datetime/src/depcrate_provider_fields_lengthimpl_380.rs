// Generated macro for impl_380 (impl)
macro_rules! Depcrate_provider_fields_lengthimpl_380 {
() => {
// Module: crate::provider::fields::length
// Provides: {"impl_380"}
// Dependencies: {}
impl FieldLength { # [inline] pub (crate) fn idx (self) -> u8 { match self { FieldLength :: One => 1 , FieldLength :: Two => 2 , FieldLength :: Three => 3 , FieldLength :: Four => 4 , FieldLength :: Five => 5 , FieldLength :: Six => 6 , FieldLength :: NumericOverride (o) => FIRST_NUMERIC_OVERRIDE . saturating_add (o as u8) . min (LAST_NUMERIC_OVERRIDE) , } } # [inline] pub (crate) fn from_idx (idx : u8) -> Result < Self , LengthError > { Ok (match idx { 1 => Self :: One , 2 => Self :: Two , 3 => Self :: Three , 4 => Self :: Four , 5 => Self :: Five , 6 => Self :: Six , idx if (FIRST_NUMERIC_OVERRIDE ..= LAST_NUMERIC_OVERRIDE) . contains (& idx) => { Self :: NumericOverride ((idx - FIRST_NUMERIC_OVERRIDE) . try_into () ?) } _ => return Err (LengthError :: InvalidLength) , }) } # [inline] pub (crate) fn to_len (self) -> usize { match self { FieldLength :: One => 1 , FieldLength :: Two => 2 , FieldLength :: Three => 3 , FieldLength :: Four => 4 , FieldLength :: Five => 5 , FieldLength :: Six => 6 , FieldLength :: NumericOverride (o) => FIRST_NUMERIC_OVERRIDE as usize + o as usize , } } # [doc = " UTS 35 defines several 1 and 2 symbols to be the same as 3 symbols (abbreviated)."] # [doc = " For example, 'a' represents an abbreviated day period, the same as 'aaa'."] # [doc = ""] # [doc = " This function maps field lengths 1 and 2 to field length 3."] pub (crate) fn numeric_to_abbr (self) -> Self { match self { FieldLength :: One | FieldLength :: Two => FieldLength :: Three , other => other , } } }
};
}
