macro_rules! flatten {
    () => {
        macro_rules ! flatten { ($ ($ T : ident) ,+) => { { # [allow (non_snake_case)] fn flatten <$ ($ T) ,+> (nest ! ($ ($ T) ,+) : nest ! ($ ($ T) ,+)) -> ($ ($ T ,) +) { ($ ($ T ,) +) } flatten } } ; }
    };
}

flatten!();