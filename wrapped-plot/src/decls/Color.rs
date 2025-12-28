macro_rules! Color {
    () => {
        # [doc = " Color"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Color { Black , Blue , Cyan , DarkViolet , ForestGreen , Gold , Gray , Green , Magenta , Red , # [doc = " Custom RGB color"] Rgb (u8 , u8 , u8) , White , Yellow , }
    };
}

Color!();