// Generated macro for App (struct)
macro_rules! DepcrateApp {
() => {
// Module: crate
// Provides: {"App"}
// Dependencies: {}
# [derive (Debug , Default)] struct App { # [doc = " The current state of the app (running or quit)"] state : AppState , # [doc = " A widget that displays the current frames per second"] fps_widget : FpsWidget , # [doc = " A widget that displays the full range of RGB colors that can be displayed in the terminal."] colors_widget : ColorsWidget , }
};
}
