macro_rules! ExpansionBehavior {
    () => {
        # [derive (Debug , Copy , Clone)] enum ExpansionBehavior { RegenerateFiles , ExpectFiles , }
    };
}

ExpansionBehavior!()