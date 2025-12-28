macro_rules! deps {
    () => {
        CrateInfo!();
    };
}

macro_rules! get_cargo_tree_data {
    () => {
        deps!();
        pub fn get_cargo_tree_data () -> & 'static [CrateInfo] { & CARGO_TREE_DATA }
    };
}

get_cargo_tree_data!();