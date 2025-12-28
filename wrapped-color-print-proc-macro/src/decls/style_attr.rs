macro_rules! deps {
    () => {
        Error!();
        Change!();
        Result!();
        Input!();
    };
}

macro_rules! style_attr {
    () => {
        deps!();
        # [doc = " Parses a style attribute."] fn style_attr (input : Input < '_ >) -> Result < '_ , Change > { let (input , word) = alpha1 (input) ? ; let change = match word { "s" | "strong" | "bold" | "em" => Change :: Bold , "dim" => Change :: Dim , "u" | "underline" => Change :: Underline , "i" | "italic" | "italics" => Change :: Italics , "blink" => Change :: Blink , "strike" => Change :: Strike , "reverse" | "rev" => Change :: Reverse , "conceal" | "hide" => Change :: Conceal , _ => { return Err (Err :: Error (Error :: new (input , ErrorKind :: Tag , None))) } } ; Ok ((input , change)) }
    };
}

style_attr!()