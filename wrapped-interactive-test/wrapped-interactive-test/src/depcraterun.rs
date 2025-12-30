// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run < W > (w : & mut W) -> Result < () > where W : Write , { execute ! (w , anes :: SwitchBufferToAlternate) ? ; enable_raw_mode () ? ; loop { queue ! (w , anes :: ResetAttributes , anes :: ClearBuffer :: All , anes :: HideCursor , anes :: MoveCursorTo (1 , 1) ,) ? ; for line in MENU . split ('\n') { queue ! (w , line , anes :: MoveCursorToNextLine (1)) ? ; } w . flush () ? ; match read_char () ? { '1' => test :: cursor :: run (w) ? , '2' => test :: color :: run (w) ? , '3' => test :: attribute :: run (w) ? , 'q' => break , _ => { } } ; } execute ! (w , anes :: ResetAttributes , anes :: ShowCursor , anes :: SwitchBufferToNormal) ? ; Ok (()) }
};
}
