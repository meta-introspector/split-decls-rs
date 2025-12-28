macro_rules! deps {
    () => {
        MatchDebugInfo!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        # [cfg (test)] impl MatchDebugInfo { pub fn match_failure_reason (& self) -> Option < & str > { self . matched . as_ref () . err () . map (| r | r . reason . as_str ()) } }
    };
}

impl_194!();