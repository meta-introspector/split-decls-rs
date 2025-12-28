macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! generate_state {
    () => {
        deps!();
        macro_rules ! generate_state { ($ state : ident , $ state_machine : tt , $ f : expr) => { loop { match $ f { Action :: None => continue , Action :: Jump (new_state) => { $ state = new_state ; continue $ state_machine ; } , Action :: End (result) => break $ state_machine result , } } } ; }
    };
}

generate_state!()