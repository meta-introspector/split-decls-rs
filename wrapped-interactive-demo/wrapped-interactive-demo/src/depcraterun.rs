// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run < W > (w : & mut W) -> io :: Result < () > where W : io :: Write , { execute ! (w , terminal :: EnterAlternateScreen) ? ; terminal :: enable_raw_mode () ? ; loop { queue ! (w , style :: ResetColor , terminal :: Clear (ClearType :: All) , cursor :: Hide , cursor :: MoveTo (1 , 1)) ? ; for line in MENU . split ('\n') { queue ! (w , style :: Print (line) , cursor :: MoveToNextLine (1)) ? ; } w . flush () ? ; match read_char () ? { '1' => test :: cursor :: run (w) ? , '2' => test :: color :: run (w) ? , '3' => test :: attribute :: run (w) ? , '4' => test :: event :: run (w) ? , '5' => test :: synchronized_output :: run (w) ? , 'q' => { execute ! (w , cursor :: SetCursorStyle :: DefaultUserShape) . unwrap () ; break ; } _ => { } } ; } execute ! (w , style :: ResetColor , cursor :: Show , terminal :: LeaveAlternateScreen) ? ; terminal :: disable_raw_mode () }
};
}
