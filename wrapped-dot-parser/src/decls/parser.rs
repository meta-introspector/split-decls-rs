macro_rules! parser {
    () => {
        mod parser { use pest_derive :: Parser ; # [derive (Parser)] # [grammar = "parser/dot.pest"] pub struct DotParser ; }
    };
}

parser!();