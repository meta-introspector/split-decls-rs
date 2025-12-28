macro_rules! Line {
    () => {
        # [doc = " A line as used in [`Event::SetInformation`](./enum.Event.html#variant.SetInformation)"] # [derive (Debug , Clone , Eq , PartialEq)] pub enum Line { # [doc = " Set a title with the given text"] Title (String) , # [doc = " Set a line of text with the given content"] Text (String) , }
    };
}

Line!()