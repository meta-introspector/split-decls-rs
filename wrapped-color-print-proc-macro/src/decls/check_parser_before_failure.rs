macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! check_parser_before_failure {
    () => {
        deps!();
        # [doc = " Checks if the first parser succeeds, then parses the input with the second parser. If an error"] # [doc = " is encountered with the second parser, then a failure message is thrown."] pub fn check_parser_before_failure < 'a , C , CV , P , PV > (mut check_parser : C , mut parser : P , failure_msg : & 'a str) -> impl Parser < 'a , PV > where C : Parser < 'a , CV > , P : Parser < 'a , PV > , { move | input | { check_parser (input) ? ; with_failure_message (| input | { parser (input) } , failure_msg) (input) } }
    };
}

check_parser_before_failure!()