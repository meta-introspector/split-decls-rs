macro_rules! Snippet {
    () => {
        pub enum Snippet { # [doc = " A tabstop snippet (e.g. `$0`)."] Tabstop (TextSize) , # [doc = " A placeholder snippet (e.g. `${0:placeholder}`)."] Placeholder (TextRange) , # [doc = " A group of placeholder snippets, e.g."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let ${0:new_var} = 4;"] # [doc = " fun(1, 2, 3, ${0:new_var});"] # [doc = " ```"] PlaceholderGroup (Vec < TextRange >) , }
    };
}

Snippet!()