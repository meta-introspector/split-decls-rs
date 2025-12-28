macro_rules! HighlightRelatedConfig {
    () => {
        # [derive (Default , Clone)] pub struct HighlightRelatedConfig { pub references : bool , pub exit_points : bool , pub break_points : bool , pub closure_captures : bool , pub yield_points : bool , pub branch_exit_points : bool , }
    };
}

HighlightRelatedConfig!();