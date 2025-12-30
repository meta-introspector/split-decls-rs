// Generated macro for ValueEnum (trait)
macro_rules! Depcrate_deriveValueEnum {
() => {
// Module: crate::derive
// Provides: {"ValueEnum"}
// Dependencies: {}
# [doc = " Parse arguments into enums."] # [doc = ""] # [doc = " When deriving [`Parser`], a field whose type implements `ValueEnum` can have the attribute"] # [doc = " `#[arg(value_enum)]` which will"] # [doc = " - Call [`EnumValueParser`][crate::builder::EnumValueParser]"] # [doc = " - Allowing using the `#[arg(default_value_t)]` attribute without implementing `Display`."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** Deriving requires the `derive` feature flag"] # [doc = ""] # [doc = " </div>"] pub trait ValueEnum : Sized + Clone { # [doc = " All possible argument values, in display order."] fn value_variants < 'a > () -> & 'a [Self] ; # [doc = " Parse an argument into `Self`."] fn from_str (input : & str , ignore_case : bool) -> Result < Self , String > { Self :: value_variants () . iter () . find (| v | { v . to_possible_value () . expect ("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value") . matches (input , ignore_case) }) . cloned () . ok_or_else (| | format ! ("invalid variant: {input}")) } # [doc = " The canonical argument value."] # [doc = ""] # [doc = " The value is `None` for skipped variants."] fn to_possible_value (& self) -> Option < PossibleValue > ; }
};
}
