macro_rules! Decomposed {
    () => {
        # [doc = " The outcome of non-recursive canonical decomposition of a character."] # [allow (clippy :: exhaustive_enums)] # [derive (Debug , PartialEq , Eq)] pub enum Decomposed { # [doc = " The character is its own canonical decomposition."] Default , # [doc = " The character decomposes to a single different character."] Singleton (char) , # [doc = " The character decomposes to two characters."] Expansion (char , char) , }
    };
}

Decomposed!();