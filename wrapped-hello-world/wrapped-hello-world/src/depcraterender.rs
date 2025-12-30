// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
# [doc = " Render the application. This is where you would draw the application UI. This example draws a"] # [doc = " greeting."] fn render (frame : & mut Frame) { let greeting = Paragraph :: new ("Hello World! (press 'q' to quit)") ; frame . render_widget (greeting , frame . area ()) ; }
};
}
