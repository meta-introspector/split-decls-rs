macro_rules! Inserted {
    () => {
        struct Inserted < V > { index : usize , old_value : Option < V > , }
    };
}

Inserted!()