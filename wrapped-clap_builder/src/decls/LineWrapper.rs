macro_rules! LineWrapper {
    () => {
        # [derive (Debug)] pub (crate) struct LineWrapper < 'w > { hard_width : usize , line_width : usize , indentation : Option < & 'w str > , }
    };
}

LineWrapper!();