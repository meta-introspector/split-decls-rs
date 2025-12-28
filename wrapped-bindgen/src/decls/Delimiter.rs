macro_rules! Delimiter {
    () => {
        # [doc = " A delimiter around a block of code"] # [derive (Copy , Clone)] pub enum Delimiter { # [doc = " `[]`"] Bracket , # [doc = " `{}`"] Brace , # [doc = " `()`"] Parenthesis , }
    };
}

Delimiter!();