macro_rules! deps {
    () => {
        Intern!();
        TrackedQuery!();
        InputQuery!();
        Transparent!();
    };
}

macro_rules! Queries {
    () => {
        deps!();
        # [allow (clippy :: large_enum_variant)] pub (crate) enum Queries { TrackedQuery (TrackedQuery) , InputQuery (InputQuery) , Intern (Intern) , Transparent (Transparent) , }
    };
}

Queries!()