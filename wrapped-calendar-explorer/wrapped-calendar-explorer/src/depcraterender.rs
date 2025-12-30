// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
# [doc = " Render the UI with a calendar."] fn render (frame : & mut Frame , calendar_style : StyledCalendar , selected_date : Date) { let header = Text :: from_iter ([Line :: from ("Calendar Example" . bold ()) , Line :: from ("<q> Quit | <s> Change Style | <n> Next Month | <p> Previous Month, <hjkl> Move" ,) , Line :: from (format ! ("Current date: {selected_date} | Current style: {calendar_style}")) ,]) ; let [text_area , area] = frame . area () . layout (& Layout :: vertical ([Constraint :: Length (header . height () as u16) , Constraint :: Fill (1) ,])) ; frame . render_widget (header . centered () , text_area) ; calendar_style . render_year (frame , area , selected_date) . unwrap () ; }
};
}
