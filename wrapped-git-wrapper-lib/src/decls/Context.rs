macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [cfg (not (feature = "anyhow_enabled"))] trait Context < T > { fn context < C > (self , _context : C) -> Result < T > where C : std :: fmt :: Display + Send + Sync + 'static ; }
    };
}

Context!()