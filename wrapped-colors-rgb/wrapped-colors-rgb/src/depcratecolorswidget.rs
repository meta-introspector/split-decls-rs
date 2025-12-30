// Generated macro for ColorsWidget (struct)
macro_rules! DepcrateColorsWidget {
() => {
// Module: crate
// Provides: {"ColorsWidget"}
// Dependencies: {}
# [doc = " A widget that displays the full range of RGB colors that can be displayed in the terminal."] # [doc = ""] # [doc = " This widget is animated and will change colors over time."] # [derive (Debug , Default)] struct ColorsWidget { # [doc = " The colors to render - should be double the height of the area as we render two rows of"] # [doc = " pixels for each row of the widget using the half block character. This is computed any time"] # [doc = " the size of the widget changes."] colors : Vec < Vec < Color > > , # [doc = " the number of elapsed frames that have passed - used to animate the colors by shifting the"] # [doc = " x index by the frame number"] frame_count : usize , }
};
}
