// Generated macro for JSON_NUMBER (const)
macro_rules! Depcrate_tagsJSON_NUMBER {
() => {
// Module: crate::tags
// Provides: {"JSON_NUMBER"}
// Dependencies: {}
# [doc = "\nA tag for numbers that are already JSON compatible.\n\nThis tag is a sub-type of [`sval::tags::NUMBER`] that:\n\n- Does not contain leading zeroes.\n- Does not use the `+` sign.\n\n# Valid datatypes\n\n- `text`\n"] pub const JSON_NUMBER : sval :: Tag = sval :: Tag :: new ("JSON_NUMBER") ;
};
}
