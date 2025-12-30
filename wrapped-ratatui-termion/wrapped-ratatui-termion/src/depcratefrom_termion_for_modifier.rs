// Generated macro for from_termion_for_modifier (macro)
macro_rules! Depcratefrom_termion_for_modifier {
() => {
// Module: crate
// Provides: {"from_termion_for_modifier"}
// Dependencies: {}
macro_rules ! from_termion_for_modifier { ($ termion_modifier : ident , $ modifier : ident) => { impl FromTermion < tstyle ::$ termion_modifier > for Modifier { fn from_termion (_ : tstyle ::$ termion_modifier) -> Self { Modifier ::$ modifier } } } ; }
};
}
