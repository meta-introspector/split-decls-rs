// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " Run the application."] fn run (terminal : & mut DefaultTerminal) -> Result < () > { let mut selected_date = OffsetDateTime :: now_local () ? . date () ; let mut calendar_style = StyledCalendar :: Default ; loop { terminal . draw (| frame | render (frame , calendar_style , selected_date)) ? ; if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Char ('q') => break Ok (()) , KeyCode :: Char ('s') => calendar_style = calendar_style . next () , KeyCode :: Char ('n') | KeyCode :: Tab => selected_date = next_month (selected_date) , KeyCode :: Char ('p') | KeyCode :: BackTab => selected_date = prev_month (selected_date) , KeyCode :: Char ('h') | KeyCode :: Left => selected_date -= 1 . days () , KeyCode :: Char ('j') | KeyCode :: Down => selected_date += 1 . weeks () , KeyCode :: Char ('k') | KeyCode :: Up => selected_date -= 1 . weeks () , KeyCode :: Char ('l') | KeyCode :: Right => selected_date += 1 . days () , _ => { } } } } }
};
}
