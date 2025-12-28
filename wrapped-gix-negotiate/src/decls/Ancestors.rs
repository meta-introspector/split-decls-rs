macro_rules! Ancestors {
    () => {
        enum Ancestors { # [doc = " Traverse only the parents of a commit."] DirectUnseen , # [doc = " Traverse all ancestors that weren't yet seen."] AllUnseen , }
    };
}

Ancestors!()