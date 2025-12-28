macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! function {
    () => {
        deps!();
        # [doc = " Creates a parser which parses a function call."] pub fn function < 'a , PV , N , P > (word_parser : N , parser : P) -> impl Parser < 'a , PV > where N : Parser < 'a , & 'a str > , P : Parser < 'a , PV > , { preceded (word (word_parser) , delimited (with_failure_message (stag ("(") , "Missing opening brace") , parser , with_failure_message (stag (")") , "Missing closing brace"))) }
    };
}

function!()