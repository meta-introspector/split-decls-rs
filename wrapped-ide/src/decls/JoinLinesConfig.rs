macro_rules! JoinLinesConfig {
    () => {
        pub struct JoinLinesConfig { pub join_else_if : bool , pub remove_trailing_comma : bool , pub unwrap_trivial_blocks : bool , pub join_assignments : bool , }
    };
}

JoinLinesConfig!();