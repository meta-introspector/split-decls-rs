macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        # [doc = " A chunk file providing a table into the parent data."] pub struct Index { # [doc = " If true, we use `chunks` in a way that facilitates writing them."] will_write : bool , # [doc = " Validated chunks as defined by their index entries."] # [doc = ""] # [doc = " Note that this list cannot be empty."] chunks : Vec < index :: Entry > , }
    };
}

Index!();