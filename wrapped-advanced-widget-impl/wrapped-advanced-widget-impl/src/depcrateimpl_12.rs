// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
# [doc = " Implement the `Widget` trait on a mutable reference to the `App` type."] # [doc = ""] # [doc = " This allows the `App` type to be rendered as a widget. The `App` type owns several other widgets"] # [doc = " that are rendered as part of the app. The `Widget` trait is implemented on a mutable reference"] # [doc = " to the `App` type, which allows this to be rendered without consuming the `App` type, and allows"] # [doc = " the sub-widgets to be mutable."] impl Widget for & mut App { fn render (self , area : Rect , buf : & mut Buffer) { let constraints = Constraint :: from_lengths ([1 , 1 , 2 , 1]) ; let [greeting , timer , squares , position] = area . layout (& Layout :: vertical (constraints)) ; Greeting :: new ("Ratatui!") . render (greeting , buf) ; self . timer . render (timer , buf) ; self . boxed_squares . render (squares , buf) ; self . green_square . render (squares , buf) ; let square_position = format ! ("Green square is at {}" , self . green_square . last_position) ; square_position . render (position , buf) ; } }
};
}
