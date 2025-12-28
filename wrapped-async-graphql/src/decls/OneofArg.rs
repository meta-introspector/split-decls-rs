macro_rules! OneofArg {
    () => {
        # [derive (OneofObject)] # [graphql (internal)] enum OneofArg { A (i32) , B (String) , }
    };
}

OneofArg!()