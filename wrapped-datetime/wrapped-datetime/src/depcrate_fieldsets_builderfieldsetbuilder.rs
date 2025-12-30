// Generated macro for FieldSetBuilder (struct)
macro_rules! Depcrate_fieldsets_builderFieldSetBuilder {
() => {
// Module: crate::fieldsets::builder
// Provides: {"FieldSetBuilder"}
// Dependencies: {}
# [doc = " A builder for [dynamic field sets](crate::fieldsets::enums)."] # [doc = ""] # [doc = " This builder is useful if you do not know the field set at code compilation time. If you do,"] # [doc = " the static field set APIs should yield smaller binary size."] # [doc = ""] # [doc = " For examples, see the [module docs](crate::fieldsets::builder)."] # [derive (Debug , Clone , PartialEq , Eq , Default)] # [non_exhaustive] pub struct FieldSetBuilder { # [doc = " The length of a formatted date/time string."] # [doc = ""] # [doc = " If `None`, defaults to [`Length::Medium`]."] pub length : Option < Length > , # [doc = " The set of date fields, such as \"year and month\" or \"weekday\"."] # [doc = ""] # [doc = " If `None`, a date will not be displayed."] pub date_fields : Option < DateFields > , # [doc = " The precision to display the time of day."] # [doc = ""] # [doc = " If `None`, a time will not be displayed."] pub time_precision : Option < TimePrecision > , # [doc = " The style to display the time zone."] # [doc = ""] # [doc = " If `None`, a time zone will not be displayed."] pub zone_style : Option < ZoneStyle > , # [doc = " The alignment context, such as when displaying dates in a table."] # [doc = ""] # [doc = " This option may be specified only if the field set can honor it."] pub alignment : Option < Alignment > , # [doc = " How to display the year and era."] # [doc = ""] # [doc = " This option may be specified only if the year is included in [`Self::date_fields`]."] pub year_style : Option < YearStyle > , }
};
}
