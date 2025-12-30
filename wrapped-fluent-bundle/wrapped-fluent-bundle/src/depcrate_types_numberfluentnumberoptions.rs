// Generated macro for FluentNumberOptions (struct)
macro_rules! Depcrate_types_numberFluentNumberOptions {
() => {
// Module: crate::types::number
// Provides: {"FluentNumberOptions"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct FluentNumberOptions { pub r#type : FluentNumberType , pub style : FluentNumberStyle , pub currency : Option < String > , pub currency_display : FluentNumberCurrencyDisplayStyle , pub use_grouping : bool , pub minimum_integer_digits : Option < usize > , pub minimum_fraction_digits : Option < usize > , pub maximum_fraction_digits : Option < usize > , pub minimum_significant_digits : Option < usize > , pub maximum_significant_digits : Option < usize > , }
};
}
