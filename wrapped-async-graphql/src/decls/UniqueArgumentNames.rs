macro_rules! UniqueArgumentNames {
    () => {
        # [derive (Default)] pub struct UniqueArgumentNames < 'a > { names : HashSet < & 'a str > , }
    };
}

UniqueArgumentNames!();