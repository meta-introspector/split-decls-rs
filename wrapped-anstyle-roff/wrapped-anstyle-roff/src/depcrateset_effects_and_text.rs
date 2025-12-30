// Generated macro for set_effects_and_text (function)
macro_rules! Depcrateset_effects_and_text {
() => {
// Module: crate
// Provides: {"set_effects_and_text"}
// Dependencies: {}
fn set_effects_and_text (styled : & StyledStr < '_ > , doc : & mut Roff) { let effects = styled . style . get_effects () ; if effects . contains (anstyle :: Effects :: BOLD) | has_bright_fg (& styled . style) { doc . text ([bold (styled . text)]) ; } else if effects . contains (anstyle :: Effects :: ITALIC) { doc . text ([italic (styled . text)]) ; } else { doc . text ([roff :: roman (styled . text)]) ; } }
};
}
