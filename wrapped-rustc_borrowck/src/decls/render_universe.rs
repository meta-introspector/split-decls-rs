macro_rules! render_universe {
    () => {
        fn render_universe (u : UniverseIndex) -> String { if u . is_root () { return "" . to_string () ; } format ! ("/{:?}" , u) }
    };
}

render_universe!()