macro_rules! deps {
    () => {
        ElementStyle!();
    };
}

macro_rules! UnderlineParts {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] pub (crate) struct UnderlineParts { pub (crate) style : ElementStyle , pub (crate) underline : char , pub (crate) label_start : char , pub (crate) vertical_text_line : char , pub (crate) multiline_vertical : char , pub (crate) multiline_horizontal : char , pub (crate) multiline_whole_line : char , pub (crate) multiline_start_down : char , pub (crate) bottom_right : char , pub (crate) top_left : char , pub (crate) top_right_flat : char , pub (crate) bottom_left : char , pub (crate) multiline_end_up : char , pub (crate) multiline_end_same_line : char , pub (crate) multiline_bottom_right_with_text : char , }
    };
}

UnderlineParts!();