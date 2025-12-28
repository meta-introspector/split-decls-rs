macro_rules! deps {
    () => {
        GitInfo!();
    };
}

macro_rules! GitDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum GitDetails { Info (GitInfo) , Error (String) , Unknown , }
    };
}

GitDetails!()