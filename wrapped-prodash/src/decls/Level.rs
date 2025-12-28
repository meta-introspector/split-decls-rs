macro_rules! Level {
    () => {
        # [doc = " a level in the hierarchy of key components"] # [doc = ""] # [doc = " _NOTE:_ This means we will show weird behaviour if there are more than 2^16 tasks at the same time on a level"] # [doc = " as multiple progress handles will manipulate the same state."] pub type Level = u8 ;
    };
}

Level!();