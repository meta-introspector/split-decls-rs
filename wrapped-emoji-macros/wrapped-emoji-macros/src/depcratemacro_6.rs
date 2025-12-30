// Generated macro for macro_6 (macro)
macro_rules! Depcratemacro_6 {
() => {
// Module: crate
// Provides: {"macro_6"}
// Dependencies: {}
define_emoji_macro ! { magnifying_glass , truth_revealed : bool , { pub fn seek_truth (& mut self) { self . truth_revealed = true ; self . energy += 25 ; println ! ("🔎 Truth revealed! Energy: {}" , self . energy) ; } pub fn obscure_truth (& mut self) { self . truth_revealed = false ; self . energy = self . energy . saturating_sub (15) ; println ! ("🙈 Truth obscured. Energy: {}" , self . energy) ; } } }
};
}
