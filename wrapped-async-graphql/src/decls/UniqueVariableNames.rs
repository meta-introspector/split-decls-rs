macro_rules! UniqueVariableNames {
    () => {
        # [derive (Default)] pub struct UniqueVariableNames < 'a > { names : HashSet < & 'a str > , }
    };
}

UniqueVariableNames!()