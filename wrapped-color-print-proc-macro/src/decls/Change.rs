macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! Change {
    () => {
        deps!();
        # [doc = " A single change to be done inside a tag. Tags with multiple keywords like `<red;bold>` will"] # [doc = " have multiple [`Change`]s."] # [derive (Debug , PartialEq , Clone)] pub enum Change { Foreground (Color) , Background (Color) , Bold , Dim , Underline , Italics , Blink , Strike , Reverse , Conceal , }
    };
}

Change!();