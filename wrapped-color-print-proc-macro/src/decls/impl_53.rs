macro_rules! deps {
    () => {
        Intensity!();
        Change!();
        Color!();
        BaseColor!();
        Color16!();
        Result!();
        Error!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl TryFrom < & str > for Change { type Error = () ; # [doc = " Tries to convert a keyword like `red`, `bold` into a [`Change`] instance."] # [rustfmt :: skip] fn try_from (input : & str) -> Result < Self , Self :: Error > { macro_rules ! color16 { ($ kind : ident $ intensity : ident $ base_color : ident) => { Change ::$ kind (Color :: Color16 (Color16 :: new (BaseColor ::$ base_color , Intensity ::$ intensity ,))) } ; } let change = match input { "s" | "strong" | "bold" | "em" => Change :: Bold , "dim" => Change :: Dim , "u" | "underline" => Change :: Underline , "i" | "italic" | "italics" => Change :: Italics , "blink" => Change :: Blink , "strike" => Change :: Strike , "reverse" | "rev" => Change :: Reverse , "conceal" | "hide" => Change :: Conceal , "k" | "black" => color16 ! (Foreground Normal Black) , "r" | "red" => color16 ! (Foreground Normal Red) , "g" | "green" => color16 ! (Foreground Normal Green) , "y" | "yellow" => color16 ! (Foreground Normal Yellow) , "b" | "blue" => color16 ! (Foreground Normal Blue) , "m" | "magenta" => color16 ! (Foreground Normal Magenta) , "c" | "cyan" => color16 ! (Foreground Normal Cyan) , "w" | "white" => color16 ! (Foreground Normal White) , "k!" | "black!" | "bright-black" => color16 ! (Foreground Bright Black) , "r!" | "red!" | "bright-red" => color16 ! (Foreground Bright Red) , "g!" | "green!" | "bright-green" => color16 ! (Foreground Bright Green) , "y!" | "yellow!" | "bright-yellow" => color16 ! (Foreground Bright Yellow) , "b!" | "blue!" | "bright-blue" => color16 ! (Foreground Bright Blue) , "m!" | "magenta!" | "bright-magenta" => color16 ! (Foreground Bright Magenta) , "c!" | "cyan!" | "bright-cyan" => color16 ! (Foreground Bright Cyan) , "w!" | "white!" | "bright-white" => color16 ! (Foreground Bright White) , "K" | "bg-black" => color16 ! (Background Normal Black) , "R" | "bg-red" => color16 ! (Background Normal Red) , "G" | "bg-green" => color16 ! (Background Normal Green) , "Y" | "bg-yellow" => color16 ! (Background Normal Yellow) , "B" | "bg-blue" => color16 ! (Background Normal Blue) , "M" | "bg-magenta" => color16 ! (Background Normal Magenta) , "C" | "bg-cyan" => color16 ! (Background Normal Cyan) , "W" | "bg-white" => color16 ! (Background Normal White) , "K!" | "bg-black!" | "bg-bright-black" => color16 ! (Background Bright Black) , "R!" | "bg-red!" | "bg-bright-red" => color16 ! (Background Bright Red) , "G!" | "bg-green!" | "bg-bright-green" => color16 ! (Background Bright Green) , "Y!" | "bg-yellow!" | "bg-bright-yellow" => color16 ! (Background Bright Yellow) , "B!" | "bg-blue!" | "bg-bright-blue" => color16 ! (Background Bright Blue) , "M!" | "bg-magenta!" | "bg-bright-magenta" => color16 ! (Background Bright Magenta) , "C!" | "bg-cyan!" | "bg-bright-cyan" => color16 ! (Background Bright Cyan) , "W!" | "bg-white!" | "bg-bright-white" => color16 ! (Background Bright White) , _ => return Err (()) , } ; Ok (change) } }
    };
}

impl_53!();