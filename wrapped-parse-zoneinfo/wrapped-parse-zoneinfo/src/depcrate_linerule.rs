// Generated macro for Rule (struct)
macro_rules! Depcrate_lineRule {
() => {
// Module: crate::line
// Provides: {"Rule"}
// Dependencies: {}
# [doc = " A **rule** definition line."] # [doc = ""] # [doc = " According to the `zic(8)` man page, a rule line has this form, along with"] # [doc = " an example:"] # [doc = ""] # [doc = " ```text"] # [doc = "     Rule  NAME  FROM  TO    TYPE  IN   ON       AT    SAVE  LETTER/S"] # [doc = "     Rule  US    1967  1973  ‐     Apr  lastSun  2:00  1:00  D"] # [doc = " ```"] # [doc = ""] # [doc = " Apart from the opening `Rule` to specify which kind of line this is, and"] # [doc = " the `type` column, every column in the line has a field in this struct."] # [derive (PartialEq , Debug , Copy , Clone)] pub struct Rule < 'a > { # [doc = " The name of the set of rules that this rule is part of."] pub name : & 'a str , # [doc = " The first year in which the rule applies."] pub from_year : Year , # [doc = " The final year, or `None` if’s ‘only’."] pub to_year : Option < Year > , # [doc = " The month in which the rule takes effect."] pub month : Month , # [doc = " The day on which the rule takes effect."] pub day : DaySpec , # [doc = " The time of day at which the rule takes effect."] pub time : TimeSpecAndType , # [doc = " The amount of time to be added when the rule is in effect."] pub time_to_add : TimeSpec , # [doc = " The variable part of time zone abbreviations to be used when this rule"] # [doc = " is in effect, if any."] pub letters : Option < & 'a str > , }
};
}
