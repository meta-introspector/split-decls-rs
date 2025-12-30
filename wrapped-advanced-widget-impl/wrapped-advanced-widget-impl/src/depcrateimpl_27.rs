// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " This widget is implemented on a mutable reference to the type, which means that it can store"] # [doc = " state and update it when it is rendered. This is useful for widgets that need to store the"] # [doc = " result of some calculation that can only be done when the widget is rendered."] # [doc = ""] # [doc = " The x and y coordinates of the square are stored in the widget and updated when the widget is"] # [doc = " rendered. This allows the square to be aligned to the right of the area. These coordinates could"] # [doc = " be used to perform hit testing (e.g. checking if a mouse click is inside the square). This app"] # [doc = " just displays the coordinates as a string."] # [doc = ""] # [doc = " This approach was probably always available in Ratatui, but it wasn't widely used either. This"] # [doc = " is an alternative to implementing the `StatefulWidget` trait, for situations where you want to"] # [doc = " store the state in the widget itself instead of a separate struct."] impl Widget for & mut RightAlignedSquare { # [doc = " Render a green square aligned to the right of the area and store the position."] fn render (self , area : Rect , buf : & mut Buffer) { const WIDTH : u16 = 4 ; let x = area . right () - WIDTH ; self . last_position = Position { x , y : area . y } ; let size = Size :: new (WIDTH , area . height) ; let area = Rect :: from ((self . last_position , size)) ; fill (area , buf , "█" , Color :: Green) ; } }
};
}
