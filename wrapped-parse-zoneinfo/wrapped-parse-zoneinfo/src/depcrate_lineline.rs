// Generated macro for Line (enum)
macro_rules! Depcrate_lineLine {
() => {
// Module: crate::line
// Provides: {"Line"}
// Dependencies: {}
# [derive (PartialEq , Debug , Copy , Clone)] pub enum Line < 'a > { # [doc = " This line is empty."] Space , # [doc = " This line contains a **zone** definition."] Zone (Zone < 'a >) , # [doc = " This line contains a **continuation** of a zone definition."] Continuation (ZoneInfo < 'a >) , # [doc = " This line contains a **rule** definition."] Rule (Rule < 'a >) , # [doc = " This line contains a **link** definition."] Link (Link < 'a >) , }
};
}
