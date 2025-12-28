macro_rules! pop_close_angle_bracket {
    () => {
        fn pop_close_angle_bracket (output : & mut String) { assert ! (output . ends_with ('>') , "'output' does not end with '>': {output}") ; output . pop () ; if output . ends_with (' ') { output . pop () ; } }
    };
}

pop_close_angle_bracket!()